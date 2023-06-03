pub type Identifier<'a> = &'a str;

pub type Script<'a> = (Vec<Block<'a>>, Vec<Statement<'a>>);

#[derive(Debug)]
pub enum Expression {
	Integer(isize),
}

#[derive(Debug)]
pub struct Block<'a>(pub Identifier<'a>, pub Vec<Instruction<'a>>);

#[derive(Debug)]
pub enum Statement<'a> {
	Successor(Identifier<'a>, (Identifier<'a>, Option<Identifier<'a>>)),
}

#[derive(Debug)]
pub enum Instruction<'a> {
	Const {
		t: Identifier<'a>,
		v: Expression,
	},
	Add {
		t: Identifier<'a>,
		l: Identifier<'a>,
		r: Identifier<'a>,
	},
	LessThanOrEqual {
		t: Identifier<'a>,
		l: Identifier<'a>,
		r: Identifier<'a>,
	},
	Test(Identifier<'a>),
	Phi {
		t: Identifier<'a>,
		l: Identifier<'a>,
		r: Identifier<'a>,
	},
	Return(Identifier<'a>),
}