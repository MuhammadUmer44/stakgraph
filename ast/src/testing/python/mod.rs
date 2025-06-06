use crate::lang::graphs::{EdgeType, NodeType};
use crate::lang::Graph;
use crate::{lang::Lang, repo::Repo};
use std::str::FromStr;
use test_log::test;

pub async fn test_python_generic<G: Graph>() -> Result<(), anyhow::Error> {
    let repo = Repo::new(
        "src/testing/python",
        Lang::from_str("python").unwrap(),
        false,
        Vec::new(),
        Vec::new(),
    )
    .unwrap();

    let graph = repo.build_graph_inner::<G>().await?;

    let (num_nodes, num_edges) = graph.get_graph_size();
    assert_eq!(num_nodes, 61, "Expected 61 nodes");
    assert_eq!(num_edges, 78, "Expected 78 edges");

    let language_nodes = graph.find_nodes_by_type(NodeType::Language);
    assert_eq!(language_nodes.len(), 1, "Expected 1 language node");
    assert_eq!(
        language_nodes[0].name, "python",
        "Language node name should be 'python'"
    );
    assert_eq!(
        language_nodes[0].file, "src/testing/python/",
        "Language node file path is incorrect"
    );

    let files = graph.find_nodes_by_type(NodeType::File);
    assert_eq!(files.len(), 16, "Expected 16 files");

    let imports = graph.find_nodes_by_type(NodeType::Import);
    assert_eq!(imports.len(), 12, "Expected 12 imports");

    let classes = graph.find_nodes_by_type(NodeType::Class);
    assert_eq!(classes.len(), 3, "Expected 3 classes");

    let mut sorted_classes = classes.clone();
    sorted_classes.sort_by(|a, b| a.name.cmp(&b.name));

    assert!(
        classes
            .iter()
            .any(|c| c.name == "Person" && c.file == "src/testing/python/model.py"),
        "Expected Person class not found"
    );

    let class_function_edges =
        graph.find_nodes_with_edge_type(NodeType::Class, NodeType::Function, EdgeType::Operand);
    assert_eq!(class_function_edges.len(), 2, "Expected 2 methods");

    let data_models = graph.find_nodes_by_type(NodeType::DataModel);
    assert_eq!(data_models.len(), 3, "Expected 3 data models");

    let endpoints = graph.find_nodes_by_type(NodeType::Endpoint);
    assert_eq!(endpoints.len(), 6, "Expected 6 endpoints");

    Ok(())
}

#[test(tokio::test)]
async fn test_python() {
    use crate::lang::graphs::{ArrayGraph, BTreeMapGraph};
    test_python_generic::<ArrayGraph>().await.unwrap();
    test_python_generic::<BTreeMapGraph>().await.unwrap();
}
