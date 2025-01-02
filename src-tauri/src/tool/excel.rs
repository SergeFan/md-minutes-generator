use std::ffi::OsString;
use std::fs::File;
use std::io::{LineWriter, Write};
use std::path::Path;

use anyhow::{anyhow, Context};
use calamine::{open_workbook, RangeDeserializerBuilder, Reader, Xlsx};
use tauri::AppHandle;
use tauri_plugin_fs::{FsExt, OpenOptions};

pub fn read_excel(input_path: &str) -> anyhow::Result<Vec<String>> {
    // Load Excel data
    let mut excel: Xlsx<_> =
        open_workbook(input_path).with_context(|| format!("Failed to open {}", input_path))?;
    let sheets = excel.worksheets();

    if sheets.is_empty() {
        return Err(anyhow!("Excel is empty, no worksheet is found"));
    }

    let sheet_names = sheets
        .iter()
        .rev()
        .map(|(sheet_name, _)| sheet_name.to_owned())
        .collect();

    Ok(sheet_names)
}

pub fn generate_markdown(
    app_handle: AppHandle,
    input_path: &str,
    output_path: &str,
    sheet_name: &str,
) -> anyhow::Result<()> {
    let mut excel: Xlsx<_> = open_workbook(input_path)
        .with_context(|| format!("Failed to open {:?}", OsString::from(input_path)))?;
    let range = excel.worksheet_range(sheet_name)?;

    let markdown_path = Path::new(output_path);
    let file_name = format!("{}_全体会.md", sheet_name.to_owned());

    let markdown_file = app_handle
        .fs()
        .open(
            markdown_path.join(&file_name),
            OpenOptions::new().write(true).create(true).to_owned(),
        )
        .with_context(|| format!("Failed to create {:?}", OsString::from(file_name)))?;

    let mut line_writer = LineWriter::new(markdown_file);

    // Load data from worksheet
    let iter = RangeDeserializerBuilder::new()
        .has_headers(false)
        .from_range(&range)?;

    for result in iter {
        let (genre, customer, project): (String, String, String) =
            result.with_context(|| "Failed to read row from Excel")?;

        let mut row = Row {
            genre,
            customer,
            project,
            line_writer: &mut line_writer,
        };

        row.write_markdown()
            .with_context(|| format!("Failed to write row to markdown template: {:?}", row))?;
    }

    Ok(())
}

#[derive(Debug)]
struct Row<'a> {
    genre: String,
    customer: String,
    project: String,
    line_writer: &'a mut LineWriter<File>,
}

impl Row<'_> {
    fn write_markdown(&mut self) -> Result<(), anyhow::Error> {
        // TODO: 現時点 `2. 財務状況報告` は `2. その他` と表示され、Excel が修正されるまで一旦文字を置換
        if !self.genre.is_empty() {
            self.line_writer.write_all(
                format!("## {}\n\n", self.genre.replace("財務状況報告", "その他")).as_bytes(),
            )?;
        }

        if !(self.customer.is_empty()
            || self.is_separator_row()
            || self.genre.contains("財務状況報告") && self.customer.contains("その他"))
        {
            self.line_writer
                .write_all(format!("### {}\n\n", self.customer.replace("\r\n", " ")).as_bytes())?;
        }

        if !(self.project.is_empty() || self.is_separator_row()) {
            self.line_writer.write_all(
                format!(
                    "#### {}\n\n",
                    self.project
                        .split('\n')
                        .map(|x| x.replace('　', " ").trim().to_string())
                        .collect::<Vec<String>>()
                        .join(" ")
                )
                .as_bytes(),
            )?;
            self.line_writer.write_all(b"- \n\n")?;
        }

        Ok(())
    }

    fn is_separator_row(&self) -> bool {
        self.genre.is_empty() && self.customer.contains("顧客名") && self.project.contains("案件名")
    }
}
