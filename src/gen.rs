use crate::parser::Node;

pub fn gen(node: Node) {
    match node {
        Node::Num(n) => {
            println!("  push {}", n);
            return;
        }
        _ => {}
    }

    match node {
        Node::Add(l, r) => {
            gen(*l);
            gen(*r);
            println!("  pop rdi");
            println!("  pop rax");

            println!("  add rax, rdi");
        }
        Node::Sub(l, r) => {
            gen(*l);
            gen(*r);
            println!("  pop rdi");
            println!("  pop rax");

            println!("  sub rax, rdi");
        }
        Node::Mul(l, r) => {
            gen(*l);
            gen(*r);
            println!("  pop rdi");
            println!("  pop rax");

            println!("  imul rax, rdi");
        }
        Node::Div(l, r) => {
            gen(*l);
            gen(*r);
            println!("  pop rdi");
            println!("  pop rax");

            println!("  cqo");
            println!("  idiv rdi");
        }
        Node::Eq(l, r) => {
            gen(*l);
            gen(*r);
            println!("  pop rdi");
            println!("  pop rax");

            println!("  cmp rax, rdi");
            println!("  sete al");
            println!("  movzb rax, al");
        }
        Node::Ne(l, r) => {
            gen(*l);
            gen(*r);
            println!("  pop rdi");
            println!("  pop rax");

            println!("  cmp rax, rdi");
            println!("  setne al");
            println!("  movzb rax, al");
        }
        Node::Lt(l, r) => {
            gen(*l);
            gen(*r);
            println!("  pop rdi");
            println!("  pop rax");

            println!("  cmp rax, rdi");
            println!("  setl al");
            println!("  movzb rax, al");
        }
        Node::Le(l, r) => {
            gen(*l);
            gen(*r);
            println!("  pop rdi");
            println!("  pop rax");

            println!("  cmp rax, rdi");
            println!("  setle al");
            println!("  movzb rax, al");
        }
        _ => {}
    }

    println!("  push rax");
}
