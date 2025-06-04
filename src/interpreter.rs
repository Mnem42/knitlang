use crate::parser::ASTNode;

pub fn run(ast: Vec<ASTNode>) {
    for node in ast {
        match node {
            ASTNode::CastOn(n) => println!("🧵 Casting on {} stitches", n),
            ASTNode::BindOff => println!("🧶 Binding off. Done!"),
            ASTNode::Knit(name) => println!("🪡 Knitting {}", name),
            ASTNode::Purl(var, val) => println!("📌 Variable {} set to \"{}\"", var, val),
            ASTNode::YarnOver => println!("🧶 Yarn over"),
            ASTNode::Repeat(times, body) => {
                println!("🔁 Repeating {} times:", times);
                for i in 0..times {
                    println!("➡️ Pass {}", i + 1);
                    run(body.clone());
                }
            }
        }
    }
}