use calamine::{open_workbook, Reader, Xlsx, DataType};
use rust_xlsxwriter::{Workbook, Format, FormatAlign};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use regex::Regex;

#[derive(Debug, Serialize, Deserialize)]
struct TimeData {
    date: String,
    time_values: HashMap<String, f64>,
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn parse_txt_file(file_path: String) -> Result<Vec<TimeData>, String> {
    let content = fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    let date_regex = Regex::new(r"日期[：:]\s*(\d{4}-\d{2}-\d{2})").unwrap();
    let time_regex = Regex::new(r"(\d{2}:\d{2})\s+(\d+(?:\.\d+)?)").unwrap();

    let mut result = Vec::new();
    let mut current_date: Option<String> = None;
    let mut current_time_values: HashMap<String, f64> = HashMap::new();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // 检查是否是日期行
        if let Some(caps) = date_regex.captures(line) {
            // 保存之前的数据
            if let Some(date) = current_date.take() {
                if !current_time_values.is_empty() {
                    result.push(TimeData {
                        date,
                        time_values: current_time_values.clone(),
                    });
                }
            }
            current_time_values.clear();
            current_date = Some(caps[1].to_string());
        } else if let Some(caps) = time_regex.captures(line) {
            // 解析时间点数据
            let time = caps[1].to_string();
            let value: f64 = caps[2]
                .parse()
                .map_err(|_| format!("Failed to parse value: {}", &caps[2]))?;
            current_time_values.insert(time, value);
        }
    }

    // 保存最后的数据
    if let Some(date) = current_date {
        if !current_time_values.is_empty() {
            result.push(TimeData {
                date,
                time_values: current_time_values,
            });
        }
    }

    Ok(result)
}

#[tauri::command]
fn read_excel_template(file_path: String) -> Result<Vec<String>, String> {
    let mut workbook: Xlsx<_> = open_workbook(&file_path)
        .map_err(|e| format!("Failed to open Excel file: {}", e))?;

    let sheet = workbook
        .worksheet_range_at(0)
        .ok_or("No worksheet found")?
        .map_err(|e| format!("Failed to read worksheet: {}", e))?;

    let mut time_columns = Vec::new();

    // 读取第一行作为列头
    if let Some(first_row) = sheet.rows().next() {
        for cell in first_row.iter().skip(1) {
            // 跳过日期列，读取时间列
            if let Some(time_str) = cell.as_string() {
                // 清理可能的格式字符
                let cleaned = time_str.trim().replace("【", "").replace("】", "");
                if !cleaned.is_empty() {
                    time_columns.push(cleaned);
                }
            }
        }
    }

    Ok(time_columns)
}

#[tauri::command]
fn convert_to_excel(
    txt_file_path: String,
    excel_template_path: String,
    output_path: String,
) -> Result<String, String> {
    // 解析txt文件
    let txt_data = parse_txt_file(txt_file_path)?;

    // 读取Excel模板获取时间列
    let time_columns = read_excel_template(excel_template_path.clone())?;

    // 创建新的Excel文件
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    // 设置格式
    let header_format = Format::new()
        .set_bold()
        .set_align(FormatAlign::Center);
    let date_format = Format::new().set_align(FormatAlign::Left);
    let number_format = Format::new().set_align(FormatAlign::Right);

    // 写入表头
    worksheet.write_string_with_format(0, 0, "日期", &header_format)
        .map_err(|e| format!("Failed to write header: {}", e))?;
    for (col_idx, time_col) in time_columns.iter().enumerate() {
        worksheet.write_string_with_format(0, (col_idx + 1) as u16, time_col, &header_format)
            .map_err(|e| format!("Failed to write header column: {}", e))?;
    }

    // 写入数据
    for (row_idx, data) in txt_data.iter().enumerate() {
        let excel_row = (row_idx + 1) as u32; // Excel行号从1开始（0是表头）

        // 写入日期
        worksheet.write_string_with_format(excel_row, 0, &data.date, &date_format)
            .map_err(|e| format!("Failed to write date: {}", e))?;

        // 写入时间点数据
        for (col_idx, time_col) in time_columns.iter().enumerate() {
            let excel_col = (col_idx + 1) as u16;
            if let Some(value) = data.time_values.get(time_col) {
                worksheet.write_number_with_format(excel_row, excel_col, *value, &number_format)
                    .map_err(|e| format!("Failed to write number: {}", e))?;
            }
            // 如果该时间点没有数据，留空（不写入）
        }
    }

    // 设置列宽
    worksheet.set_column_width(0, 15.0)
        .map_err(|e| format!("Failed to set column width: {}", e))?;
    for i in 1..=time_columns.len() {
        worksheet.set_column_width(i as u16, 12.0)
            .map_err(|e| format!("Failed to set column width: {}", e))?;
    }

    // 保存文件
    workbook.save(&output_path)
        .map_err(|e| format!("Failed to save Excel file: {}", e))?;

    Ok(output_path)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            parse_txt_file,
            read_excel_template,
            convert_to_excel
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
