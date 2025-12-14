use orion_error::ErrorOwe;
use rusqlite::Params;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use wp_error::KnowledgeResult;
use wp_log::debug_kdb;
use wp_model_core::model::{self, DataField};

use lazy_static::lazy_static;

lazy_static! {
    /// Global column-name cache keyed by raw SQL text, shared by MemDB queries.
    pub static ref COLNAME_CACHE: RwLock<HashMap<String, Arc<Vec<String>>>> =
        RwLock::new(HashMap::new());
}

/// Query first row and map columns into Vec<DataField> with column names preserved.
pub fn query_first_row<P: Params>(
    conn: &rusqlite::Connection,
    sql: &str,
    params: P,
) -> KnowledgeResult<Vec<DataField>> {
    let mut stmt = conn.prepare_cached(sql).owe_rule()?;
    let col_cnt = stmt.column_count();
    debug_kdb!("[memdb] col_cnt={}", col_cnt);
    // Precompute column names before row borrow
    let mut col_names: Vec<String> = Vec::with_capacity(col_cnt);
    for i in 0..col_cnt {
        let name = stmt.column_name(i).unwrap_or("").to_string();
        debug_kdb!("[memdb] col[{}] name='{}'", i, name);
        col_names.push(name);
    }
    let mut rows = stmt.query(params).owe_rule()?;
    let mut result = Vec::new();
    if let Some(row) = rows.next().owe_rule()? {
        for (i, col_name) in col_names.iter().enumerate().take(col_cnt) {
            let x = row.get_ref(i).owe_rule()?;
            let col_name = col_name.clone();
            match x {
                rusqlite::types::ValueRef::Null => {
                    result.push(DataField::new(
                        model::DataType::default(),
                        col_name,
                        model::Value::Null,
                    ));
                }
                rusqlite::types::ValueRef::Integer(v) => {
                    result.push(DataField::from_digit(col_name, v));
                }
                rusqlite::types::ValueRef::Real(v) => {
                    result.push(DataField::from_float(col_name, v));
                }
                rusqlite::types::ValueRef::Text(v) => {
                    result.push(DataField::from_chars(
                        col_name,
                        String::from_utf8(v.to_vec()).owe_rule()?,
                    ));
                }
                rusqlite::types::ValueRef::Blob(v) => {
                    let s = String::from_utf8_lossy(v).to_string();
                    result.push(DataField::from_chars(col_name, s));
                }
            }
        }
    } else {
        debug_kdb!("[memdb] no row for sql");
    }
    Ok(result)
}

/// Same as `query_first_row` but with a shared column-names cache to reduce metadata lookups.
pub fn query_first_row_cached<P: Params>(
    conn: &rusqlite::Connection,
    sql: &str,
    params: P,
) -> KnowledgeResult<Vec<DataField>> {
    let mut stmt = conn.prepare_cached(sql).owe_rule()?;
    let col_cnt = stmt.column_count();
    // Column names cache (per SQL)
    let col_names: Vec<String> =
        if let Some(names) = COLNAME_CACHE.read().ok().and_then(|m| m.get(sql).cloned()) {
            (*names).clone()
        } else {
            let mut names = Vec::with_capacity(col_cnt);
            for i in 0..col_cnt {
                names.push(stmt.column_name(i).owe_rule()?.to_string());
            }
            if let Ok(mut m) = COLNAME_CACHE.write() {
                m.insert(sql.to_string(), Arc::new(names.clone()));
            }
            names
        };
    let mut rows = stmt.query(params).owe_rule()?;
    let mut result = Vec::new();
    if let Some(row) = rows.next().owe_rule()? {
        for (i, col_name) in col_names.iter().enumerate().take(col_cnt) {
            let x = row.get_ref(i).owe_rule()?;
            let col_name = col_name.clone();
            match x {
                rusqlite::types::ValueRef::Null => {
                    result.push(DataField::new(
                        model::DataType::default(),
                        col_name,
                        model::Value::Null,
                    ));
                }
                rusqlite::types::ValueRef::Integer(v) => {
                    result.push(DataField::from_digit(col_name, v));
                }
                rusqlite::types::ValueRef::Real(v) => {
                    result.push(DataField::from_float(col_name, v));
                }
                rusqlite::types::ValueRef::Text(v) => {
                    result.push(DataField::from_chars(
                        col_name,
                        String::from_utf8(v.to_vec()).owe_rule()?,
                    ));
                }
                rusqlite::types::ValueRef::Blob(v) => {
                    let s = String::from_utf8_lossy(v).to_string();
                    result.push(DataField::from_chars(col_name, s));
                }
            }
        }
    }
    Ok(result)
}
