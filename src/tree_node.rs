#[derive(Debug)]
pub struct TreeNode {
    pub val: String,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

pub fn eval(root: &TreeNode, v: f64) -> f64 {
    let val = &root.val[..];

    // is val a number?
    if let Ok(a) =  val.parse() {
        return a;
    }


    let l = match root.left {
        Some(ref x) => eval(x, v),
        _ => 0.0
    };

    let r = match root.right {
        Some(ref x) => eval(x, v),
        _ => 0.0
    };


    match val {
        "t" => v,
        "pi" => std::f64::consts::PI,
        "e" => std::f64::consts::E,
        "sin" => l.sin(),
        "cos" => l.cos(),
        "tan" => l.tan(),
        "ln" => l.ln(),
        "exp" => l.exp(),
        "sqrt" => l.sqrt(),
        "+" => l + r,
        "-" => l - r,
        "*" => l * r,
        "/" => l / r,
        "**" => l.powf(r),
        _ => f64::MIN
    }
}

#[test]
fn test_eval() {
    let x = Some(Box::new(TreeNode {
        val: "-".to_string(),
        left: Some(Box::new(
            TreeNode {
                val: "+".to_string(),
                left: Some(Box::new(
                    TreeNode {
                        val: "*".to_string(),
                        left: Some(Box::new(
                            TreeNode {
                                val: "t".to_string(),
                                left: None,
                                right: None,
                            },
                        )),
                        right: Some(Box::new(
                            TreeNode {
                                val: "2".to_string(),
                                left: None,
                                right: None,
                            },
                        )),
                    },
                )),
                right: Some(Box::new(
                    TreeNode {
                        val: "t".to_string(),
                        left: None,
                        right: None,
                    },
                )),
            },
        )),
        right: Some(Box::new(
            TreeNode {
                val: "exp".to_string(),
                left: Some(Box::new(
                    TreeNode {
                        val: "t".to_string(),
                        left: None,
                        right: None,
                    },
                )),
                right: None,
            },
        )),
    }));

    assert_eq!(eval(&x.as_ref().unwrap(), 1.0), 0.2817181715409549);
    assert_eq!(eval(&x.as_ref().unwrap(), 2.0),  -1.3890560989306504);
}