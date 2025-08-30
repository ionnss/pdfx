// exporter.rs

// exporter.rs

use crate::types::PdfListResult;

use serde_json;
use serde_yaml;
use std::fs::File;
use std::io::Write;

pub struct Exporters;

impl Exporters {
    pub fn export_to_json(
        pdfs: &[PdfListResult],
        export_path: &std::path::Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let json_data = serde_json::to_string_pretty(pdfs)?;
        let file_path = export_path.join("pdfs.json");
        let mut file = File::create(file_path)?;
        file.write_all(json_data.as_bytes())?;
        Ok(())
    }

    pub fn export_to_csv(
        pdfs: &[PdfListResult],
        export_path: &std::path::Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut csv_data = String::new();
        csv_data.push_str("id,filename,size,path,modified\n");

        for pdf in pdfs {
            // Properly escape CSV fields
            let id = pdf.id.unwrap_or(0);
            let filename = format!("\"{}\"", pdf.filename.replace("\"", "\"\""));
            let path = format!("\"{}\"", pdf.path.replace("\"", "\"\""));
            let modified = pdf.modified.format("%Y-%m-%d %H:%M:%S");

            csv_data.push_str(&format!(
                "{},{},{},{},{}\n",
                id, filename, pdf.size_human, path, modified
            ));
        }

        let file_path = export_path.join("pdfs.csv");
        let mut file = File::create(file_path)?;
        file.write_all(csv_data.as_bytes())?;
        Ok(())
    }

    pub fn export_to_markdown(
        pdfs: &[PdfListResult],
        export_path: &std::path::Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut md_data = String::new();
        md_data.push_str("# PDF Library Report\n\n");
        md_data.push_str(&format!("## Summary\n"));
        md_data.push_str(&format!("- **Total PDFs**: {}\n", pdfs.len()));
        md_data.push_str(&format!(
            "- **Export Date**: {}\n\n",
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S")
        ));

        md_data.push_str("## PDFs\n\n");
        md_data.push_str("| # | Filename | Size | Path | Modified |\n");
        md_data.push_str("|---|----------|------|------|----------|\n");

        for (i, pdf) in pdfs.iter().enumerate() {
            md_data.push_str(&format!(
                "| {} | {} | {} | {} | {} |\n",
                i + 1,
                pdf.filename,
                pdf.size_human,
                pdf.path,
                pdf.modified.format("%Y-%m-%d %H:%M:%S")
            ));
        }

        let file_path = export_path.join("pdfs.md");
        let mut file = File::create(file_path)?;
        file.write_all(md_data.as_bytes())?;
        Ok(())
    }

    pub fn export_to_yaml(
        pdfs: &[PdfListResult],
        export_path: &std::path::Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let yaml_data = serde_yaml::to_string(pdfs)?;
        let file_path = export_path.join("pdfs.yaml");
        let mut file = File::create(file_path)?;
        file.write_all(yaml_data.as_bytes())?;
        Ok(())
    }

    pub fn export_to_html(
        pdfs: &[PdfListResult],
        export_path: &std::path::Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut html_data = String::new();
        html_data
            .push_str("<!DOCTYPE html>\n<html>\n<head><title>PDF Library</title></head>\n<body>\n");
        html_data.push_str("<h1>PDF Library</h1>\n<table>\n");
        html_data.push_str("<tr><th>Filename</th><th>Size</th><th>Modified</th></tr>\n");

        for pdf in pdfs {
            html_data.push_str(&format!(
                "<tr><td>{}</td><td>{}</td><td>{}</td></tr>\n",
                pdf.filename,
                pdf.size_human,
                pdf.modified.format("%Y-%m-%d %H:%M:%S")
            ));
        }

        html_data.push_str("</table>\n</body>\n</html>");

        let file_path = export_path.join("pdfs.html");
        let mut file = File::create(file_path)?;
        file.write_all(html_data.as_bytes())?;
        Ok(())
    }
}
