// Libraries
use std::error;

// Nodes for the syntax tree
#[derive(Debug, Clone)]
pub enum Node {
    /*  
        Almost each of the variants include 2 nodes,
        so the tree can be built.

        i.e 2+2*3
        Add(2, Multiply(2, 3)) 
    */
    Add(Box<Node>, Box<Node>),
    Subtract(Box<Node>, Box<Node>),
    Multiply(Box<Node>, Box<Node>),
    Divide(Box<Node>, Box<Node>),
    Pow(Box<Node>, Box<Node>),
    // only includes the negative value
    NegativeValue(Box<Node>),
    // number values
    Number(f64),
    // strings
    String(String)
}