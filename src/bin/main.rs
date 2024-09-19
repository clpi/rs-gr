use graph::AppGraph;
use ll::*;
#[tokio::main]
async fn main() {
    println!("Hi");
    let mut g: AppGraph<String, String> = AppGraph::new();

    let a = g.add_node("a".into());
    let b = g.add_node("b".into());
    let c = g.add_node("c".into());
    let d = g.add_node("d".into());
    let e = g.add_node("e".into());
    let f = g.add_node("f".into());
    let g = g.add_node("g".into());

    let ab = g.add_edge(a, b, "likes".into());
    let ae = g.add_edge(a, e, "hates".into());
    let bc = g.add_edge(b, c, "is amazed by".into());
    let ba = g.add_edge(b, d, "fancies".into());
    let bd = g.add_edge(b, d, "is not fond of".into());
    let be = g.add_edge(b, e, "dislikes".into());
    let cf = g.add_edge(b, f, "likes a lot".into());
    let cg = g.add_edge(b, f, "fancies".into());
    let cb = g.add_edge(b, f, "is amused by".into());
    let dc = g.add_edge(b, f, "laughs at".into());
    let da = g.add_edge(b, f, "cares for".into());
    let dg = g.add_edge(b, f, "is curious about".into());

    let f = g.write_file().await.unwrap();
    let fg = AppGraph::<String, String>::read_file(f).await.unwrap();
    println!("{:?}", fg);
}
