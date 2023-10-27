use std::fmt::Debug;

pub trait TreeNode: Debug {
    fn print(&self) -> String;
    fn eval(&self) -> isize;
}

#[derive(Debug)]
pub struct Constant {
    val: isize,
    symbol: &'static str
}

impl TreeNode for Constant {
    fn print(&self) -> String {
        format!("{}", self.symbol)
    }

    fn eval(&self) -> isize {
        self.val
    }
}

#[derive(Debug)]
pub struct Add {
    pub left: Box<dyn TreeNode>,
    pub right: Box<dyn TreeNode>,
}

impl TreeNode for Add {
    fn print(&self) -> String {
        format!("({} + {})", self.left.print(), self.right.print())
    }

    fn eval(&self) -> isize {
        self.left.eval() + self.right.eval()
    }
}
#[derive(Debug)]
pub struct Subtract {
    pub left: Box<dyn TreeNode>,
    pub right: Box<dyn TreeNode>,
}

impl TreeNode for Subtract {
    fn print(&self) -> String {
        format!("({} - {})", self.left.print(), self.right.print())
    }

    fn eval(&self) -> isize {
        self.left.eval() - self.right.eval()
    }
}

#[derive(Debug)]
pub struct Negate {
    pub arg: Box<dyn TreeNode>,
}

impl TreeNode for Negate {
    fn print(&self) -> String {
        format!("-({})", self.arg.print())
    }

    fn eval(&self) -> isize {
        -(self.arg.eval())
    }
}

#[derive(Debug)]
pub struct Factorial {
    pub arg: Box<dyn TreeNode>,
}

impl TreeNode for Factorial {
    fn print(&self) -> String {
        format!("({})!", self.arg.print())
    }

    fn eval(&self) -> isize {
        (1..=self.arg.eval()).product()
    }
}

#[derive(Debug)]
pub struct Integer {
    pub val: isize,
}

impl TreeNode for Integer {
    fn print(&self) -> String {
        format!("{}", self.val)
    }

    fn eval(&self) -> isize {
        self.val
    }
}

#[derive(Debug)]
pub struct Mult {
    pub left: Box<dyn TreeNode>,
    pub right: Box<dyn TreeNode>,
}

impl TreeNode for Mult {
    fn print(&self) -> String {
        format!("({} * {})", self.left.print(), self.right.print())
    }

    fn eval(&self) -> isize {
        self.left.eval() * self.right.eval()
    }
}

#[derive(Debug)]
pub struct Div {
    pub left: Box<dyn TreeNode>,
    pub right: Box<dyn TreeNode>,
}

impl TreeNode for Div {
    fn print(&self) -> String {
        format!("({} / ({}))", self.left.print(), self.right.print())
    }

    fn eval(&self) -> isize {
        self.left.eval() / self.right.eval()
    }
}

#[derive(Debug)]
pub struct Pow {
    pub left: Box<dyn TreeNode>,
    pub right: Box<dyn TreeNode>,
}

impl TreeNode for Pow {
    fn print(&self) -> String {
        format!("({} ^ ({}))", self.left.print(), self.right.print())
    }

    fn eval(&self) -> isize {
        isize::pow(self.left.eval(), self.right.eval().try_into().unwrap())
    }
}
