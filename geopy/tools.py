def parse_xl_path(file_path):
    if file_path.endswith((".xlsx", ".xls")):
        return file_path, None
    if "\\" not in file_path:
        raise ValueError("Invalid path, sheet name missing")
    excel_path, sheet_name = file_path.rsplit("\\", 1)
    if not excel_path.endswith((".xlsx", ".xls")):
        raise ValueError(f"Invalid Excel file: {excel_path}")
    return excel_path, sheet_name