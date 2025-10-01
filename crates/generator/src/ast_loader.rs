use mongodb::sync::{Client, Collection};
use mongodb::bson::{doc, Document};
use anyhow::Result;
use std::collections::HashMap;

pub fn load_ASTs() -> Result<Vec<Vec<Document>>> {
    let client = Client::with_uri_str("mongodb://localhost:27017/")?;
    let db = client.database("runtime-fuzz");

    // 获取所有集合名称，需要调用 run() 方法
    let collection_names = db.list_collection_names().run()?;
    let mut all_docs_per_collection: Vec<Vec<Document>> = Vec::new();

    for col_name in collection_names {
        let collection: Collection<Document> = db.collection(&col_name);

        // find() 方法只接受一个参数，返回 Find 结构体，需要调用 run() 执行
        let cursor = collection.find(doc! {}).run()?;
        let docs: Vec<Document> = cursor.filter_map(|r| r.ok()).collect();

        if !docs.is_empty() {
            all_docs_per_collection.push(docs);
        }
    }

    Ok(all_docs_per_collection)
}

pub fn load_ASTs_hashmap() -> Result<HashMap<String, Vec<Document>>> {
    let client = Client::with_uri_str("mongodb://localhost:27017/")?;
    let db = client.database("runtime-fuzz");

    let collection_names = db.list_collection_names().run()?;
    let mut all_docs_per_collection: HashMap<String, Vec<Document>> = HashMap::new();

    for col_name in collection_names {
        let collection: Collection<Document> = db.collection(&col_name);
        let cursor = collection.find(doc! {}).run()?;
        let docs: Vec<Document> = cursor.filter_map(|r| r.ok()).collect();

        if !docs.is_empty() {
            all_docs_per_collection.insert(col_name.clone(), docs);
        }
    }

    Ok(all_docs_per_collection)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_all_runtime_fuzz_documents() {
        let result = load_ASTs_hashmap();
        assert!(result.is_ok(), "函数执行失败: {:?}", result.err());
        
        let collections_docs = result.unwrap();
        println!("找到 {} 个集合", collections_docs.len());
        
        for (col_name, docs) in &collections_docs {
            println!("集合 '{}': 包含 {} 个文档", col_name, docs.len());
        }
        
        println!("测试通过：成功加载了 {} 个集合的数据", collections_docs.len());
    }
}