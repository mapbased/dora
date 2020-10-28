use std::fmt;
use std::slice::Iter;
use std::sync::Arc;

use crate::ast::Elem::*;
use crate::interner::{Interner, Name};
use crate::lexer::position::{Position, Span};
use crate::lexer::token::{FloatSuffix, IntBase, IntSuffix};

pub mod dump;
pub mod visit;

#[derive(Clone, Debug)]
pub struct File {
    pub path: String,
    pub content: String,
    pub line_ends: Vec<u32>,
    pub elements: Vec<Elem>,
}

impl File {
    #[cfg(test)]
    pub fn fct0(&self) -> &Function {
        self.elements[0].to_function().unwrap()
    }

    #[cfg(test)]
    pub fn fct(&self, index: usize) -> &Function {
        self.elements[index].to_function().unwrap()
    }

    #[cfg(test)]
    pub fn cls0(&self) -> &Class {
        self.elements[0].to_class().unwrap()
    }

    #[cfg(test)]
    pub fn cls(&self, index: usize) -> &Class {
        self.elements[index].to_class().unwrap()
    }

    #[cfg(test)]
    pub fn struct0(&self) -> &Struct {
        self.elements[0].to_struct().unwrap()
    }

    #[cfg(test)]
    pub fn enum0(&self) -> &Enum {
        self.elements[0].to_enum().unwrap()
    }

    #[cfg(test)]
    pub fn alias0(&self) -> &Alias {
        self.elements[0].to_alias().unwrap()
    }

    #[cfg(test)]
    pub fn namespace0(&self) -> &Namespace {
        self.elements[0].to_namespace().unwrap()
    }

    #[cfg(test)]
    pub fn trai(&self, index: usize) -> &Trait {
        self.elements[index].to_trait().unwrap()
    }

    #[cfg(test)]
    pub fn trait0(&self) -> &Trait {
        self.elements[0].to_trait().unwrap()
    }

    #[cfg(test)]
    pub fn impl0(&self) -> &Impl {
        self.elements[0].to_impl().unwrap()
    }

    #[cfg(test)]
    pub fn mod0(&self) -> &Module {
        self.elements[0].to_module().unwrap()
    }

    #[cfg(test)]
    pub fn modu(&self, index: usize) -> &Module {
        self.elements[index].to_module().unwrap()
    }

    #[cfg(test)]
    pub fn ann0(&self) -> &Annotation {
        self.elements[0].to_annotation().unwrap()
    }

    #[cfg(test)]
    pub fn global0(&self) -> &Global {
        self.elements[0].to_global().unwrap()
    }

    #[cfg(test)]
    pub fn const0(&self) -> &Const {
        self.elements[0].to_const().unwrap()
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub struct NodeId(pub usize);

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{}", self.0)
    }
}

#[derive(Clone, Debug)]
pub enum Elem {
    ElemFunction(Arc<Function>),
    ElemClass(Arc<Class>),
    ElemStruct(Struct),
    ElemTrait(Trait),
    ElemImpl(Impl),
    ElemModule(Module),
    ElemAnnotation(Annotation),
    ElemGlobal(Global),
    ElemConst(Const),
    ElemEnum(Enum),
    ElemAlias(Alias),
    ElemNamespace(Namespace),
}

impl Elem {
    pub fn id(&self) -> NodeId {
        match self {
            &ElemFunction(ref f) => f.id,
            &ElemClass(ref c) => c.id,
            &ElemStruct(ref s) => s.id,
            &ElemTrait(ref t) => t.id,
            &ElemImpl(ref i) => i.id,
            &ElemModule(ref m) => m.id,
            &ElemAnnotation(ref a) => a.id,
            &ElemGlobal(ref g) => g.id,
            &ElemConst(ref c) => c.id,
            &ElemEnum(ref e) => e.id,
            &ElemAlias(ref e) => e.id,
            &ElemNamespace(ref e) => e.id,
        }
    }

    pub fn to_function(&self) -> Option<&Function> {
        match self {
            &ElemFunction(ref fct) => Some(fct),
            _ => None,
        }
    }

    pub fn to_class(&self) -> Option<&Class> {
        match self {
            &ElemClass(ref class) => Some(class),
            _ => None,
        }
    }

    pub fn to_enum(&self) -> Option<&Enum> {
        match self {
            &ElemEnum(ref xenum) => Some(xenum),
            _ => None,
        }
    }

    pub fn to_alias(&self) -> Option<&Alias> {
        match self {
            &ElemAlias(ref alias) => Some(alias),
            _ => None,
        }
    }

    pub fn to_namespace(&self) -> Option<&Namespace> {
        match self {
            &ElemNamespace(ref namespace) => Some(namespace),
            _ => None,
        }
    }

    pub fn to_struct(&self) -> Option<&Struct> {
        match self {
            &ElemStruct(ref struc) => Some(struc),
            _ => None,
        }
    }

    pub fn to_trait(&self) -> Option<&Trait> {
        match self {
            &ElemTrait(ref trai) => Some(trai),
            _ => None,
        }
    }

    pub fn to_impl(&self) -> Option<&Impl> {
        match self {
            &ElemImpl(ref ximpl) => Some(ximpl),
            _ => None,
        }
    }

    pub fn to_module(&self) -> Option<&Module> {
        match self {
            &ElemModule(ref module) => Some(module),
            _ => None,
        }
    }

    pub fn to_annotation(&self) -> Option<&Annotation> {
        match self {
            &ElemAnnotation(ref annotation) => Some(annotation),
            _ => None,
        }
    }

    pub fn to_global(&self) -> Option<&Global> {
        match self {
            &ElemGlobal(ref global) => Some(global),
            _ => None,
        }
    }

    pub fn to_const(&self) -> Option<&Const> {
        match self {
            &ElemConst(ref konst) => Some(konst),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Global {
    pub id: NodeId,
    pub pos: Position,
    pub span: Span,
    pub name: Name,
    pub reassignable: bool,
    pub data_type: Type,
    pub initializer: Option<Arc<Function>>,
}

#[derive(Clone, Debug)]
pub struct Namespace {
    pub id: NodeId,
    pub pos: Position,
    pub span: Span,
    pub name: Name,
    pub elements: Vec<Elem>,
}

#[derive(Clone, Debug)]
pub struct Const {
    pub id: NodeId,
    pub pos: Position,
    pub span: Span,
    pub name: Name,
    pub data_type: Type,
    pub expr: Box<Expr>,
}

#[derive(Clone, Debug)]
pub struct Enum {
    pub id: NodeId,
    pub pos: Position,
    pub span: Span,
    pub name: Name,
    pub type_params: Option<Vec<TypeParam>>,
    pub variants: Vec<EnumVariant>,
}

#[derive(Clone, Debug)]
pub struct EnumVariant {
    pub id: NodeId,
    pub pos: Position,
    pub span: Span,
    pub name: Name,
    pub types: Option<Vec<Type>>,
}

#[derive(Clone, Debug)]
pub struct Alias {
    pub id: NodeId,
    pub pos: Position,
    pub span: Span,
    pub name: Name,
    pub ty: Type,
}

#[derive(Clone, Debug)]
pub struct Struct {
    pub id: NodeId,
    pub pos: Position,
    pub span: Span,
    pub name: Name,
    pub fields: Vec<StructField>,
}

#[derive(Clone, Debug)]
pub struct StructField {
    pub id: NodeId,
    pub name: Name,
    pub pos: Position,
    pub span: Span,
    pub data_type: Type,
}

#[derive(Clone, Debug)]
pub enum Type {
    TypeSelf(TypeSelfType),
    TypeBasic(TypeBasicType),
    TypeTuple(TypeTupleType),
    TypeLambda(TypeLambdaType),
}

#[derive(Clone, Debug)]
pub struct TypeSelfType {
    pub id: NodeId,
    pub pos: Position,
    pub span: Span,
}

#[derive(Clone, Debug)]
pub struct TypeTupleType {
    pub id: NodeId,
    pub pos: Position,
    pub span: Span,

    pub subtypes: Vec<Box<Type>>,
}

#[derive(Clone, Debug)]
pub struct TypeLambdaType {
    pub id: NodeId,
    pub pos: Position,
    pub span: Span,

    pub params: Vec<Box<Type>>,
    pub ret: Box<Type>,
}

#[derive(Clone, Debug)]
pub struct TypeBasicType {
    pub id: NodeId,
    pub pos: Position,
    pub span: Span,

    pub name: Name,
    pub params: Vec<Box<Type>>,
}

impl Type {
    pub fn create_self(id: NodeId, pos: Position, span: Span) -> Type {
        Type::TypeSelf(TypeSelfType { id, pos, span })
    }

    pub fn create_basic(
        id: NodeId,
        pos: Position,
        span: Span,
        name: Name,
        params: Vec<Box<Type>>,
    ) -> Type {
        Type::TypeBasic(TypeBasicType {
            id,
            pos,
            span,
            name,
            params,
        })
    }

    pub fn create_fct(
        id: NodeId,
        pos: Position,
        span: Span,
        params: Vec<Box<Type>>,
        ret: Box<Type>,
    ) -> Type {
        Type::TypeLambda(TypeLambdaType {
            id,
            pos,
            span,
            params,
            ret,
        })
    }

    pub fn create_tuple(id: NodeId, pos: Position, span: Span, subtypes: Vec<Box<Type>>) -> Type {
        Type::TypeTuple(TypeTupleType {
            id,
            pos,
            span,
            subtypes,
        })
    }

    pub fn to_basic(&self) -> Option<&TypeBasicType> {
        match *self {
            Type::TypeBasic(ref val) => Some(val),
            _ => None,
        }
    }

    pub fn to_basic_without_type_params(&self) -> Option<Name> {
        match *self {
            Type::TypeBasic(ref basic) => {
                if basic.params.len() == 0 {
                    Some(basic.name)
                } else {
                    None
                }
            }

            _ => None,
        }
    }

    pub fn to_tuple(&self) -> Option<&TypeTupleType> {
        match *self {
            Type::TypeTuple(ref val) => Some(val),
            _ => None,
        }
    }

    pub fn to_fct(&self) -> Option<&TypeLambdaType> {
        match *self {
            Type::TypeLambda(ref val) => Some(val),
            _ => None,
        }
    }

    #[cfg(test)]
    pub fn is_unit(&self) -> bool {
        match self {
            &Type::TypeTuple(ref val) if val.subtypes.len() == 0 => true,
            _ => false,
        }
    }

    pub fn to_string(&self, interner: &Interner) -> String {
        match *self {
            Type::TypeSelf(_) => "Self".into(),
            Type::TypeBasic(ref val) => format!("{}", *interner.str(val.name)),

            Type::TypeTuple(ref val) => {
                let types: Vec<String> =
                    val.subtypes.iter().map(|t| t.to_string(interner)).collect();

                format!("({})", types.join(", "))
            }

            Type::TypeLambda(ref val) => {
                let types: Vec<String> = val.params.iter().map(|t| t.to_string(interner)).collect();
                let ret = val.ret.to_string(interner);

                format!("({}) -> {}", types.join(", "), ret)
            }
        }
    }

    pub fn pos(&self) -> Position {
        match *self {
            Type::TypeSelf(ref val) => val.pos,
            Type::TypeBasic(ref val) => val.pos,
            Type::TypeTuple(ref val) => val.pos,
            Type::TypeLambda(ref val) => val.pos,
        }
    }

    pub fn id(&self) -> NodeId {
        match *self {
            Type::TypeSelf(ref val) => val.id,
            Type::TypeBasic(ref val) => val.id,
            Type::TypeTuple(ref val) => val.id,
            Type::TypeLambda(ref val) => val.id,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Impl {
    pub id: NodeId,
    pub pos: Position,
    pub span: Span,

    pub type_params: Option<Vec<TypeParam>>,
    pub trait_type: Option<Type>,
    pub class_type: Type,
    pub methods: Vec<Arc<Function>>,
}

#[derive(Clone, Debug)]
pub struct Trait {
    pub id: NodeId,
    pub name: Name,
    pub pos: Position,
    pub span: Span,
    pub methods: Vec<Arc<Function>>,
}

#[derive(Clone, Debug)]
pub struct Class {
    pub id: NodeId,
    pub name: Name,
    pub pos: Position,
    pub span: Span,
    pub parent_class: Option<ParentClass>,
    pub has_open: bool,
    pub is_abstract: bool,
    pub internal: bool,
    pub has_constructor: bool,

    pub constructor: Option<Arc<Function>>,
    pub fields: Vec<Field>,
    pub methods: Vec<Arc<Function>>,
    pub initializers: Vec<Box<Stmt>>,
    pub type_params: Option<Vec<TypeParam>>,
}

#[derive(Clone, Debug)]
pub struct Module {
    pub id: NodeId,
    pub name: Name,
    pub pos: Position,
    pub parent_class: Option<ParentClass>,
    pub internal: bool,
    pub has_constructor: bool,

    pub constructor: Option<Arc<Function>>,
    pub fields: Vec<Field>,
    pub methods: Vec<Arc<Function>>,
    pub initializers: Vec<Box<Stmt>>,
}

#[derive(Clone, Debug)]
pub struct Annotation {
    pub id: NodeId,
    pub name: Name,
    pub pos: Position,
    pub annotation_usages: AnnotationUsages,
    pub internal: Option<Modifier>,

    pub type_params: Option<Vec<TypeParam>>,
    pub term_params: Option<Vec<AnnotationParam>>,
}

#[derive(Clone, Debug)]
pub struct TypeParam {
    pub name: Name,
    pub pos: Position,
    pub span: Span,
    pub bounds: Vec<Type>,
}

#[derive(Clone, Debug)]
pub struct ConstructorParam {
    pub name: Name,
    pub pos: Position,
    pub span: Span,
    pub data_type: Type,
    pub field: bool,
    pub reassignable: bool,
    pub variadic: bool,
}

#[derive(Clone, Debug)]
pub struct AnnotationParam {
    pub name: Name,
    pub pos: Position,
    pub span: Span,
    pub data_type: Type,
}

#[derive(Clone, Debug)]
pub struct ParentClass {
    pub name: Name,
    pub pos: Position,
    pub span: Span,
    pub type_params: Vec<Type>,
    pub params: Vec<Box<Expr>>,
}

impl ParentClass {
    pub fn new(
        name: Name,
        pos: Position,
        span: Span,
        type_params: Vec<Type>,
        params: Vec<Box<Expr>>,
    ) -> ParentClass {
        ParentClass {
            name,
            pos,
            span,
            type_params,
            params,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Field {
    pub id: NodeId,
    pub name: Name,
    pub pos: Position,
    pub span: Span,
    pub data_type: Type,
    pub primary_ctor: bool,
    pub expr: Option<Box<Expr>>,
    pub reassignable: bool,
}

#[derive(Clone, Debug)]
pub struct Function {
    pub id: NodeId,
    pub name: Name,
    pub pos: Position,
    pub span: Span,
    pub method: bool,
    pub has_open: bool,
    pub has_override: bool,
    pub has_final: bool,
    pub has_optimize_immediately: bool,
    pub is_pub: bool,
    pub is_static: bool,
    pub is_abstract: bool,
    pub is_test: bool,
    pub use_cannon: bool,
    pub internal: bool,
    pub is_constructor: bool,
    pub newcall: bool,

    pub params: Vec<Param>,

    pub return_type: Option<Type>,
    pub block: Option<Box<ExprBlockType>>,
    pub type_params: Option<Vec<TypeParam>>,
}

impl Function {
    pub fn block(&self) -> &ExprBlockType {
        self.block.as_ref().unwrap()
    }
}

// remove in next step
#[derive(Clone, Debug)]
pub struct Modifiers(Vec<ModifierElement>);

// remove in next step
impl Modifiers {
    pub fn new() -> Modifiers {
        Modifiers(Vec::new())
    }

    pub fn contains(&self, modifier: Modifier) -> bool {
        self.0.iter().find(|el| el.value == modifier).is_some()
    }

    pub fn add(&mut self, modifier: Modifier, pos: Position, span: Span) {
        self.0.push(ModifierElement {
            value: modifier,
            pos,
            span,
        });
    }

    pub fn iter(&self) -> Iter<ModifierElement> {
        self.0.iter()
    }
}

// remove in next step
#[derive(Clone, Debug)]
pub struct ModifierElement {
    pub value: Modifier,
    pub pos: Position,
    pub span: Span,
}

#[derive(Clone, Debug)]
pub struct AnnotationUsages(Vec<AnnotationUsage>);

impl AnnotationUsages {
    pub fn new() -> AnnotationUsages {
        AnnotationUsages(Vec::new())
    }

    pub fn contains(&self, name: Name) -> bool {
        self.0.iter().find(|el| el.name == name).is_some()
    }

    pub fn add(&mut self, annotation_usage: AnnotationUsage) {
        self.0.push(annotation_usage);
    }

    pub fn iter(&self) -> Iter<AnnotationUsage> {
        self.0.iter()
    }
}

#[derive(Clone, Debug)]
pub struct AnnotationUsage {
    pub name: Name,
    pub pos: Position,
    pub span: Span,
    pub type_args: Vec<Type>,
    pub term_args: Vec<Box<Expr>>,
}

// rename to InternalAnnotation in next step
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Modifier {
    Abstract,
    Override,
    Open,
    Final,
    Internal,
    Pub,
    Static,
    Test,
    Cannon,
    OptimizeImmediately,
    NewCall,
}

impl Modifier {
    pub fn find(name: &str) -> Option<Modifier> {
        match name {
            "abstract" => Some(Modifier::Abstract),
            "open" => Some(Modifier::Open),
            "override" => Some(Modifier::Override),
            "final" => Some(Modifier::Final),
            "internal" => Some(Modifier::Internal),
            "pub" => Some(Modifier::Pub),
            "static" => Some(Modifier::Static),
            "test" => Some(Modifier::Test),
            "cannon" => Some(Modifier::Cannon),
            "optimizeImmediately" => Some(Modifier::OptimizeImmediately),
            "newcall" => Some(Modifier::NewCall),
            _ => None,
        }
    }

    pub fn name(&self) -> &'static str {
        match *self {
            Modifier::Abstract => "abstract",
            Modifier::Open => "open",
            Modifier::Override => "override",
            Modifier::Final => "final",
            Modifier::Internal => "internal",
            Modifier::Pub => "pub",
            Modifier::Static => "static",
            Modifier::Test => "test",
            Modifier::Cannon => "cannon",
            Modifier::OptimizeImmediately => "optimizeImmediately",
            Modifier::NewCall => "newcall",
        }
    }
}

#[derive(Clone, Debug)]
pub struct Param {
    pub id: NodeId,
    pub idx: u32,
    pub name: Name,
    pub pos: Position,
    pub span: Span,
    pub data_type: Type,
    pub variadic: bool,
}

#[derive(Clone, Debug)]
pub enum Stmt {
    StmtLet(StmtLetType),
    StmtWhile(StmtWhileType),
    StmtExpr(StmtExprType),
    StmtBreak(StmtBreakType),
    StmtContinue(StmtContinueType),
    StmtReturn(StmtReturnType),
    StmtFor(StmtForType),
}

impl Stmt {
    pub fn create_let(
        id: NodeId,
        pos: Position,
        span: Span,
        pattern: Box<LetPattern>,
        reassignable: bool,
        data_type: Option<Type>,
        expr: Option<Box<Expr>>,
    ) -> Stmt {
        Stmt::StmtLet(StmtLetType {
            id,
            pos,
            span,

            pattern,
            reassignable,
            data_type,
            expr,
        })
    }

    pub fn create_for(
        id: NodeId,
        pos: Position,
        span: Span,
        pattern: Box<LetPattern>,
        expr: Box<Expr>,
        block: Box<Stmt>,
    ) -> Stmt {
        Stmt::StmtFor(StmtForType {
            id,
            pos,
            span,

            pattern,
            expr,
            block,
        })
    }

    pub fn create_while(
        id: NodeId,
        pos: Position,
        span: Span,
        cond: Box<Expr>,
        block: Box<Stmt>,
    ) -> Stmt {
        Stmt::StmtWhile(StmtWhileType {
            id,
            pos,
            span,

            cond,
            block,
        })
    }

    pub fn create_expr(id: NodeId, pos: Position, span: Span, expr: Box<Expr>) -> Stmt {
        Stmt::StmtExpr(StmtExprType {
            id,
            pos,
            span,

            expr,
        })
    }

    pub fn create_break(id: NodeId, pos: Position, span: Span) -> Stmt {
        Stmt::StmtBreak(StmtBreakType { id, pos, span })
    }

    pub fn create_continue(id: NodeId, pos: Position, span: Span) -> Stmt {
        Stmt::StmtContinue(StmtContinueType { id, pos, span })
    }

    pub fn create_return(id: NodeId, pos: Position, span: Span, expr: Option<Box<Expr>>) -> Stmt {
        Stmt::StmtReturn(StmtReturnType {
            id,
            pos,
            span,

            expr,
        })
    }

    pub fn id(&self) -> NodeId {
        match *self {
            Stmt::StmtLet(ref stmt) => stmt.id,
            Stmt::StmtWhile(ref stmt) => stmt.id,
            Stmt::StmtFor(ref stmt) => stmt.id,
            Stmt::StmtExpr(ref stmt) => stmt.id,
            Stmt::StmtBreak(ref stmt) => stmt.id,
            Stmt::StmtContinue(ref stmt) => stmt.id,
            Stmt::StmtReturn(ref stmt) => stmt.id,
        }
    }

    pub fn pos(&self) -> Position {
        match *self {
            Stmt::StmtLet(ref stmt) => stmt.pos,
            Stmt::StmtWhile(ref stmt) => stmt.pos,
            Stmt::StmtFor(ref stmt) => stmt.pos,
            Stmt::StmtExpr(ref stmt) => stmt.pos,
            Stmt::StmtBreak(ref stmt) => stmt.pos,
            Stmt::StmtContinue(ref stmt) => stmt.pos,
            Stmt::StmtReturn(ref stmt) => stmt.pos,
        }
    }

    pub fn span(&self) -> Span {
        match *self {
            Stmt::StmtLet(ref stmt) => stmt.span,
            Stmt::StmtWhile(ref stmt) => stmt.span,
            Stmt::StmtFor(ref stmt) => stmt.span,
            Stmt::StmtExpr(ref stmt) => stmt.span,
            Stmt::StmtBreak(ref stmt) => stmt.span,
            Stmt::StmtContinue(ref stmt) => stmt.span,
            Stmt::StmtReturn(ref stmt) => stmt.span,
        }
    }

    pub fn to_let(&self) -> Option<&StmtLetType> {
        match *self {
            Stmt::StmtLet(ref val) => Some(val),
            _ => None,
        }
    }

    pub fn is_let(&self) -> bool {
        match *self {
            Stmt::StmtLet(_) => true,
            _ => false,
        }
    }

    pub fn to_while(&self) -> Option<&StmtWhileType> {
        match *self {
            Stmt::StmtWhile(ref val) => Some(val),
            _ => None,
        }
    }

    pub fn is_while(&self) -> bool {
        match *self {
            Stmt::StmtWhile(_) => true,
            _ => false,
        }
    }

    pub fn to_for(&self) -> Option<&StmtForType> {
        match *self {
            Stmt::StmtFor(ref val) => Some(val),
            _ => None,
        }
    }

    pub fn is_for(&self) -> bool {
        match *self {
            Stmt::StmtFor(_) => true,
            _ => false,
        }
    }

    pub fn to_expr(&self) -> Option<&StmtExprType> {
        match *self {
            Stmt::StmtExpr(ref val) => Some(val),
            _ => None,
        }
    }

    pub fn is_expr(&self) -> bool {
        match *self {
            Stmt::StmtExpr(_) => true,
            _ => false,
        }
    }

    pub fn to_return(&self) -> Option<&StmtReturnType> {
        match *self {
            Stmt::StmtReturn(ref val) => Some(val),
            _ => None,
        }
    }

    pub fn is_return(&self) -> bool {
        match *self {
            Stmt::StmtReturn(_) => true,
            _ => false,
        }
    }

    pub fn to_break(&self) -> Option<&StmtBreakType> {
        match *self {
            Stmt::StmtBreak(ref val) => Some(val),
            _ => None,
        }
    }

    pub fn is_break(&self) -> bool {
        match *self {
            Stmt::StmtBreak(_) => true,
            _ => false,
        }
    }

    pub fn to_continue(&self) -> Option<&StmtContinueType> {
        match *self {
            Stmt::StmtContinue(ref val) => Some(val),
            _ => None,
        }
    }

    pub fn is_continue(&self) -> bool {
        match *self {
            Stmt::StmtContinue(_) => true,
            _ => false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct StmtLetType {
    pub id: NodeId,
    pub pos: Position,
    pub span: Span,

    pub pattern: Box<LetPattern>,
    pub reassignable: bool,

    pub data_type: Option<Type>,
    pub expr: Option<Box<Expr>>,
}

#[derive(Clone, Debug)]
pub enum LetPattern {
    Ident(LetIdentType),
    Tuple(LetTupleType),
    Underscore(LetUnderscoreType),
}

impl LetPattern {
    pub fn is_ident(&self) -> bool {
        match self {
            LetPattern::Ident(_) => true,
            _ => false,
        }
    }

    pub fn is_tuple(&self) -> bool {
        match self {
            LetPattern::Tuple(_) => true,
            _ => false,
        }
    }

    pub fn is_underscore(&self) -> bool {
        match self {
            LetPattern::Underscore(_) => true,
            _ => false,
        }
    }

    pub fn to_name(&self) -> Option<Name> {
        match self {
            LetPattern::Ident(ref ident) => Some(ident.name),
            _ => None,
        }
    }

    pub fn to_ident(&self) -> Option<&LetIdentType> {
        match self {
            LetPattern::Ident(ref ident) => Some(ident),
            _ => None,
        }
    }

    pub fn to_tuple(&self) -> Option<&LetTupleType> {
        match self {
            LetPattern::Tuple(ref tuple) => Some(tuple),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct LetUnderscoreType {
    pub id: NodeId,
    pub pos: Position,
    pub span: Span,
}

#[derive(Clone, Debug)]
pub struct LetIdentType {
    pub id: NodeId,
    pub pos: Position,
    pub span: Span,
    pub mutable: bool,
    pub name: Name,
}

#[derive(Clone, Debug)]
pub struct LetTupleType {
    pub id: NodeId,
    pub pos: Position,
    pub span: Span,
    pub parts: Vec<Box<LetPattern>>,
}

#[derive(Clone, Debug)]
pub struct StmtForType {
    pub id: NodeId,
    pub pos: Position,
    pub span: Span,

    pub pattern: Box<LetPattern>,
    pub expr: Box<Expr>,
    pub block: Box<Stmt>,
}

#[derive(Clone, Debug)]
pub struct StmtWhileType {
    pub id: NodeId,
    pub pos: Position,
    pub span: Span,

    pub cond: Box<Expr>,
    pub block: Box<Stmt>,
}

#[derive(Clone, Debug)]
pub struct StmtExprType {
    pub id: NodeId,
    pub pos: Position,
    pub span: Span,

    pub expr: Box<Expr>,
}

#[derive(Clone, Debug)]
pub struct StmtReturnType {
    pub id: NodeId,
    pub pos: Position,
    pub span: Span,

    pub expr: Option<Box<Expr>>,
}

#[derive(Clone, Debug)]
pub struct StmtBreakType {
    pub id: NodeId,
    pub pos: Position,
    pub span: Span,
}

#[derive(Clone, Debug)]
pub struct StmtContinueType {
    pub id: NodeId,
    pub pos: Position,
    pub span: Span,
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum UnOp {
    Plus,
    Neg,
    Not,
}

impl UnOp {
    pub fn as_str(&self) -> &'static str {
        match *self {
            UnOp::Plus => "+",
            UnOp::Neg => "-",
            UnOp::Not => "!",
        }
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum CmpOp {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    Is,
    IsNot,
}

impl CmpOp {
    pub fn as_str(&self) -> &'static str {
        match *self {
            CmpOp::Eq => "==",
            CmpOp::Ne => "!=",
            CmpOp::Lt => "<",
            CmpOp::Le => "<=",
            CmpOp::Gt => ">",
            CmpOp::Ge => ">=",
            CmpOp::Is => "===",
            CmpOp::IsNot => "!==",
        }
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum BinOp {
    Assign,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Cmp(CmpOp),
    Or,
    And,
    BitOr,
    BitAnd,
    BitXor,
    ShiftL,
    ArithShiftR,
    LogicalShiftR,
}

impl BinOp {
    pub fn as_str(&self) -> &'static str {
        match *self {
            BinOp::Assign => "=",
            BinOp::Add => "+",
            BinOp::Sub => "-",
            BinOp::Mul => "*",
            BinOp::Div => "/",
            BinOp::Mod => "%",
            BinOp::Cmp(op) => op.as_str(),
            BinOp::Or => "||",
            BinOp::And => "&&",
            BinOp::BitOr => "|",
            BinOp::BitAnd => "&",
            BinOp::BitXor => "^",
            BinOp::ShiftL => "<<",
            BinOp::ArithShiftR => ">>",
            BinOp::LogicalShiftR => ">>>",
        }
    }

    pub fn is_any_assign(&self) -> bool {
        match *self {
            BinOp::Assign => true,
            _ => false,
        }
    }

    pub fn is_compare(&self) -> bool {
        match *self {
            BinOp::Cmp(cmp) if cmp != CmpOp::Is && cmp != CmpOp::IsNot => true,
            _ => false,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Expr {
    ExprUn(ExprUnType),
    ExprBin(ExprBinType),
    ExprLitChar(ExprLitCharType),
    ExprLitInt(ExprLitIntType),
    ExprLitFloat(ExprLitFloatType),
    ExprLitStr(ExprLitStrType),
    ExprTemplate(ExprTemplateType),
    ExprLitBool(ExprLitBoolType),
    ExprIdent(ExprIdentType),
    ExprCall(ExprCallType),
    ExprTypeParam(ExprTypeParamType),
    ExprPath(ExprPathType),
    ExprDelegation(ExprDelegationType),
    ExprDot(ExprDotType),
    ExprSelf(ExprSelfType),
    ExprSuper(ExprSuperType),
    ExprConv(ExprConvType),
    ExprLambda(ExprLambdaType),
    ExprBlock(ExprBlockType),
    ExprIf(ExprIfType),
    ExprTuple(ExprTupleType),
    ExprParen(ExprParenType),
    ExprMatch(ExprMatchType),
}

impl Expr {
    pub fn create_block(
        id: NodeId,
        pos: Position,
        span: Span,
        stmts: Vec<Box<Stmt>>,
        expr: Option<Box<Expr>>,
    ) -> Expr {
        Expr::ExprBlock(ExprBlockType {
            id,
            pos,
            span,

            stmts,
            expr,
        })
    }

    pub fn create_if(
        id: NodeId,
        pos: Position,
        span: Span,
        cond: Box<Expr>,
        then_block: Box<Expr>,
        else_block: Option<Box<Expr>>,
    ) -> Expr {
        Expr::ExprIf(ExprIfType {
            id,
            pos,
            span,

            cond,
            then_block,
            else_block,
        })
    }

    pub fn create_un(id: NodeId, pos: Position, span: Span, op: UnOp, opnd: Box<Expr>) -> Expr {
        Expr::ExprUn(ExprUnType {
            id,
            pos,
            span,

            op,
            opnd,
        })
    }

    pub fn create_bin(
        id: NodeId,
        pos: Position,
        span: Span,
        op: BinOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    ) -> Expr {
        Expr::ExprBin(ExprBinType {
            id,
            pos,
            span,

            op,
            initializer: false,
            lhs,
            rhs,
        })
    }

    pub fn create_conv(
        id: NodeId,
        pos: Position,
        span: Span,
        object: Box<Expr>,
        data_type: Box<Type>,
        is: bool,
    ) -> Expr {
        Expr::ExprConv(ExprConvType {
            id,
            pos,
            span,

            object,
            data_type,
            is,
        })
    }

    pub fn create_lit_char(id: NodeId, pos: Position, span: Span, value: char) -> Expr {
        Expr::ExprLitChar(ExprLitCharType {
            id,
            pos,
            span,

            value,
        })
    }

    pub fn create_lit_int(
        id: NodeId,
        pos: Position,
        span: Span,
        value: u64,
        base: IntBase,
        suffix: IntSuffix,
    ) -> Expr {
        Expr::ExprLitInt(ExprLitIntType {
            id,
            pos,
            span,

            value,
            base,
            suffix,
        })
    }

    pub fn create_lit_float(
        id: NodeId,
        pos: Position,
        span: Span,
        value: f64,
        suffix: FloatSuffix,
    ) -> Expr {
        Expr::ExprLitFloat(ExprLitFloatType {
            id,
            pos,
            span,
            value,
            suffix,
        })
    }

    pub fn create_lit_str(id: NodeId, pos: Position, span: Span, value: String) -> Expr {
        Expr::ExprLitStr(ExprLitStrType {
            id,
            pos,
            span,

            value,
        })
    }

    pub fn create_template(id: NodeId, pos: Position, span: Span, parts: Vec<Box<Expr>>) -> Expr {
        Expr::ExprTemplate(ExprTemplateType {
            id,
            pos,
            span,

            parts,
        })
    }

    pub fn create_lit_bool(id: NodeId, pos: Position, span: Span, value: bool) -> Expr {
        Expr::ExprLitBool(ExprLitBoolType {
            id,
            pos,
            span,

            value,
        })
    }

    pub fn create_this(id: NodeId, pos: Position, span: Span) -> Expr {
        Expr::ExprSelf(ExprSelfType { id, pos, span })
    }

    pub fn create_super(id: NodeId, pos: Position, span: Span) -> Expr {
        Expr::ExprSuper(ExprSuperType { id, pos, span })
    }

    pub fn create_ident(
        id: NodeId,
        pos: Position,
        span: Span,
        name: Name,
        type_params: Option<Vec<Type>>,
    ) -> Expr {
        Expr::ExprIdent(ExprIdentType {
            id,
            pos,
            span,

            name,
            type_params,
        })
    }

    pub fn create_paren(id: NodeId, pos: Position, span: Span, expr: Box<Expr>) -> Expr {
        Expr::ExprParen(ExprParenType {
            id,
            pos,
            span,

            expr,
        })
    }

    pub fn create_call(
        id: NodeId,
        pos: Position,
        span: Span,
        callee: Box<Expr>,
        args: Vec<Box<Expr>>,
    ) -> Expr {
        Expr::ExprCall(ExprCallType {
            id,
            pos,
            span,

            callee,
            args,
        })
    }

    pub fn create_type_param(
        id: NodeId,
        pos: Position,
        span: Span,
        callee: Box<Expr>,
        args: Vec<Type>,
    ) -> Expr {
        Expr::ExprTypeParam(ExprTypeParamType {
            id,
            pos,
            span,

            callee,
            args,
        })
    }

    pub fn create_path(
        id: NodeId,
        pos: Position,
        span: Span,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    ) -> Expr {
        Expr::ExprPath(ExprPathType {
            id,
            pos,
            span,

            lhs,
            rhs,
        })
    }

    pub fn create_delegation(id: NodeId, pos: Position, span: Span, args: Vec<Box<Expr>>) -> Expr {
        Expr::ExprDelegation(ExprDelegationType {
            id,
            pos,
            span,

            args,
        })
    }

    pub fn create_dot(
        id: NodeId,
        pos: Position,
        span: Span,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    ) -> Expr {
        Expr::ExprDot(ExprDotType {
            id,
            pos,
            span,

            lhs,
            rhs,
        })
    }

    pub fn create_lambda(
        id: NodeId,
        pos: Position,
        span: Span,
        params: Vec<Param>,
        ret: Option<Box<Type>>,
        block: Box<Stmt>,
    ) -> Expr {
        Expr::ExprLambda(ExprLambdaType {
            id,
            pos,
            span,

            params,
            ret,
            block,
        })
    }

    pub fn create_tuple(id: NodeId, pos: Position, span: Span, values: Vec<Box<Expr>>) -> Expr {
        Expr::ExprTuple(ExprTupleType {
            id,
            pos,
            span,
            values,
        })
    }

    pub fn to_un(&self) -> Option<&ExprUnType> {
        match *self {
            Expr::ExprUn(ref val) => Some(val),
            _ => None,
        }
    }

    pub fn is_un(&self) -> bool {
        match *self {
            Expr::ExprUn(_) => true,
            _ => false,
        }
    }

    pub fn to_bin(&self) -> Option<&ExprBinType> {
        match *self {
            Expr::ExprBin(ref val) => Some(val),
            _ => None,
        }
    }

    pub fn is_bin(&self) -> bool {
        match *self {
            Expr::ExprBin(_) => true,
            _ => false,
        }
    }

    pub fn to_paren(&self) -> Option<&ExprParenType> {
        match *self {
            Expr::ExprParen(ref val) => Some(val),
            _ => None,
        }
    }

    pub fn is_paren(&self) -> bool {
        match *self {
            Expr::ExprParen(_) => true,
            _ => false,
        }
    }

    pub fn to_ident(&self) -> Option<&ExprIdentType> {
        match *self {
            Expr::ExprIdent(ref val) => Some(val),
            _ => None,
        }
    }

    pub fn is_ident(&self) -> bool {
        match *self {
            Expr::ExprIdent(_) => true,
            _ => false,
        }
    }

    pub fn to_call(&self) -> Option<&ExprCallType> {
        match *self {
            Expr::ExprCall(ref val) => Some(val),
            _ => None,
        }
    }

    pub fn is_call(&self) -> bool {
        match *self {
            Expr::ExprCall(_) => true,
            _ => false,
        }
    }

    pub fn to_path(&self) -> Option<&ExprPathType> {
        match *self {
            Expr::ExprPath(ref val) => Some(val),
            _ => None,
        }
    }

    pub fn is_path(&self) -> bool {
        match *self {
            Expr::ExprPath(_) => true,
            _ => false,
        }
    }

    pub fn to_type_param(&self) -> Option<&ExprTypeParamType> {
        match *self {
            Expr::ExprTypeParam(ref val) => Some(val),
            _ => None,
        }
    }

    pub fn is_type_param(&self) -> bool {
        match *self {
            Expr::ExprTypeParam(_) => true,
            _ => false,
        }
    }

    pub fn to_lit_char(&self) -> Option<&ExprLitCharType> {
        match *self {
            Expr::ExprLitChar(ref val) => Some(val),
            _ => None,
        }
    }

    pub fn is_lit_char(&self) -> bool {
        match *self {
            Expr::ExprLitChar(_) => true,
            _ => false,
        }
    }

    pub fn to_lit_int(&self) -> Option<&ExprLitIntType> {
        match *self {
            Expr::ExprLitInt(ref val) => Some(val),
            _ => None,
        }
    }

    pub fn is_lit_int(&self) -> bool {
        match *self {
            Expr::ExprLitInt(_) => true,
            _ => false,
        }
    }

    pub fn to_template(&self) -> Option<&ExprTemplateType> {
        match *self {
            Expr::ExprTemplate(ref val) => Some(val),
            _ => None,
        }
    }

    pub fn is_template(&self) -> bool {
        match *self {
            Expr::ExprTemplate(_) => true,
            _ => false,
        }
    }

    pub fn to_lit_float(&self) -> Option<&ExprLitFloatType> {
        match *self {
            Expr::ExprLitFloat(ref val) => Some(val),
            _ => None,
        }
    }

    pub fn is_lit_float(&self) -> bool {
        match *self {
            Expr::ExprLitFloat(_) => true,
            _ => false,
        }
    }

    pub fn to_lit_str(&self) -> Option<&ExprLitStrType> {
        match *self {
            Expr::ExprLitStr(ref val) => Some(val),
            _ => None,
        }
    }

    pub fn is_lit_str(&self) -> bool {
        match *self {
            Expr::ExprLitStr(_) => true,
            _ => false,
        }
    }

    pub fn to_lit_bool(&self) -> Option<&ExprLitBoolType> {
        match *self {
            Expr::ExprLitBool(ref val) => Some(val),
            _ => None,
        }
    }

    pub fn is_lit_bool(&self) -> bool {
        match *self {
            Expr::ExprLitBool(_) => true,
            _ => false,
        }
    }

    pub fn is_lit_true(&self) -> bool {
        match *self {
            Expr::ExprLitBool(ref lit) if lit.value => true,
            _ => false,
        }
    }

    pub fn to_dot(&self) -> Option<&ExprDotType> {
        match *self {
            Expr::ExprDot(ref val) => Some(val),
            _ => None,
        }
    }

    pub fn is_dot(&self) -> bool {
        match *self {
            Expr::ExprDot(_) => true,
            _ => false,
        }
    }

    pub fn to_delegation(&self) -> Option<&ExprDelegationType> {
        match *self {
            Expr::ExprDelegation(ref val) => Some(val),
            _ => None,
        }
    }

    pub fn is_delegation(&self) -> bool {
        match *self {
            Expr::ExprDelegation(_) => true,
            _ => false,
        }
    }

    pub fn is_this(&self) -> bool {
        match *self {
            Expr::ExprSelf(_) => true,
            _ => false,
        }
    }

    pub fn is_super(&self) -> bool {
        match *self {
            Expr::ExprSuper(_) => true,
            _ => false,
        }
    }

    pub fn to_super(&self) -> Option<&ExprSuperType> {
        match *self {
            Expr::ExprSuper(ref val) => Some(val),
            _ => None,
        }
    }

    pub fn to_conv(&self) -> Option<&ExprConvType> {
        match *self {
            Expr::ExprConv(ref val) => Some(val),
            _ => None,
        }
    }

    pub fn is_conv(&self) -> bool {
        match *self {
            Expr::ExprConv(_) => true,
            _ => false,
        }
    }

    pub fn to_lambda(&self) -> Option<&ExprLambdaType> {
        match *self {
            Expr::ExprLambda(ref val) => Some(val),
            _ => None,
        }
    }

    pub fn is_lambda(&self) -> bool {
        match self {
            &Expr::ExprLambda(_) => true,
            _ => false,
        }
    }

    pub fn to_tuple(&self) -> Option<&ExprTupleType> {
        match *self {
            Expr::ExprTuple(ref val) => Some(val),
            _ => None,
        }
    }

    pub fn is_tuple(&self) -> bool {
        match *self {
            Expr::ExprTuple(_) => true,
            _ => false,
        }
    }

    pub fn to_block(&self) -> Option<&ExprBlockType> {
        match *self {
            Expr::ExprBlock(ref val) => Some(val),
            _ => None,
        }
    }

    pub fn is_block(&self) -> bool {
        match self {
            &Expr::ExprBlock(_) => true,
            _ => false,
        }
    }

    pub fn to_if(&self) -> Option<&ExprIfType> {
        match *self {
            Expr::ExprIf(ref val) => Some(val),
            _ => None,
        }
    }

    pub fn is_if(&self) -> bool {
        match *self {
            Expr::ExprIf(_) => true,
            _ => false,
        }
    }

    pub fn needs_semicolon(&self) -> bool {
        match self {
            &Expr::ExprBlock(_) => false,
            &Expr::ExprIf(_) => false,
            _ => true,
        }
    }

    pub fn pos(&self) -> Position {
        match *self {
            Expr::ExprUn(ref val) => val.pos,
            Expr::ExprBin(ref val) => val.pos,
            Expr::ExprLitChar(ref val) => val.pos,
            Expr::ExprLitInt(ref val) => val.pos,
            Expr::ExprLitFloat(ref val) => val.pos,
            Expr::ExprLitStr(ref val) => val.pos,
            Expr::ExprTemplate(ref val) => val.pos,
            Expr::ExprLitBool(ref val) => val.pos,
            Expr::ExprIdent(ref val) => val.pos,
            Expr::ExprCall(ref val) => val.pos,
            Expr::ExprTypeParam(ref val) => val.pos,
            Expr::ExprPath(ref val) => val.pos,
            Expr::ExprDelegation(ref val) => val.pos,
            Expr::ExprDot(ref val) => val.pos,
            Expr::ExprSelf(ref val) => val.pos,
            Expr::ExprSuper(ref val) => val.pos,
            Expr::ExprConv(ref val) => val.pos,
            Expr::ExprLambda(ref val) => val.pos,
            Expr::ExprBlock(ref val) => val.pos,
            Expr::ExprIf(ref val) => val.pos,
            Expr::ExprTuple(ref val) => val.pos,
            Expr::ExprParen(ref val) => val.pos,
            Expr::ExprMatch(ref val) => val.pos,
        }
    }

    pub fn span(&self) -> Span {
        match *self {
            Expr::ExprUn(ref val) => val.span,
            Expr::ExprBin(ref val) => val.span,
            Expr::ExprLitChar(ref val) => val.span,
            Expr::ExprLitInt(ref val) => val.span,
            Expr::ExprLitFloat(ref val) => val.span,
            Expr::ExprLitStr(ref val) => val.span,
            Expr::ExprTemplate(ref val) => val.span,
            Expr::ExprLitBool(ref val) => val.span,
            Expr::ExprIdent(ref val) => val.span,
            Expr::ExprCall(ref val) => val.span,
            Expr::ExprTypeParam(ref val) => val.span,
            Expr::ExprPath(ref val) => val.span,
            Expr::ExprDelegation(ref val) => val.span,
            Expr::ExprDot(ref val) => val.span,
            Expr::ExprSelf(ref val) => val.span,
            Expr::ExprSuper(ref val) => val.span,
            Expr::ExprConv(ref val) => val.span,
            Expr::ExprLambda(ref val) => val.span,
            Expr::ExprBlock(ref val) => val.span,
            Expr::ExprIf(ref val) => val.span,
            Expr::ExprTuple(ref val) => val.span,
            Expr::ExprParen(ref val) => val.span,
            Expr::ExprMatch(ref val) => val.span,
        }
    }

    pub fn id(&self) -> NodeId {
        match *self {
            Expr::ExprUn(ref val) => val.id,
            Expr::ExprBin(ref val) => val.id,
            Expr::ExprLitChar(ref val) => val.id,
            Expr::ExprLitInt(ref val) => val.id,
            Expr::ExprLitFloat(ref val) => val.id,
            Expr::ExprLitStr(ref val) => val.id,
            Expr::ExprTemplate(ref val) => val.id,
            Expr::ExprLitBool(ref val) => val.id,
            Expr::ExprIdent(ref val) => val.id,
            Expr::ExprCall(ref val) => val.id,
            Expr::ExprTypeParam(ref val) => val.id,
            Expr::ExprPath(ref val) => val.id,
            Expr::ExprDelegation(ref val) => val.id,
            Expr::ExprDot(ref val) => val.id,
            Expr::ExprSelf(ref val) => val.id,
            Expr::ExprSuper(ref val) => val.id,
            Expr::ExprConv(ref val) => val.id,
            Expr::ExprLambda(ref val) => val.id,
            Expr::ExprBlock(ref val) => val.id,
            Expr::ExprIf(ref val) => val.id,
            Expr::ExprTuple(ref val) => val.id,
            Expr::ExprParen(ref val) => val.id,
            Expr::ExprMatch(ref val) => val.id,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ExprIfType {
    pub id: NodeId,
    pub pos: Position,
    pub span: Span,

    pub cond: Box<Expr>,
    pub then_block: Box<Expr>,
    pub else_block: Option<Box<Expr>>,
}

#[derive(Clone, Debug)]
pub struct ExprTupleType {
    pub id: NodeId,
    pub pos: Position,
    pub span: Span,

    pub values: Vec<Box<Expr>>,
}

#[derive(Clone, Debug)]
pub struct ExprConvType {
    pub id: NodeId,
    pub pos: Position,
    pub span: Span,

    pub object: Box<Expr>,
    pub is: bool,
    pub data_type: Box<Type>,
}

#[derive(Clone, Debug)]
pub struct ExprDelegationType {
    pub id: NodeId,
    pub pos: Position,
    pub span: Span,

    pub args: Vec<Box<Expr>>,
}

#[derive(Clone, Debug)]
pub struct ExprUnType {
    pub id: NodeId,
    pub pos: Position,
    pub span: Span,

    pub op: UnOp,
    pub opnd: Box<Expr>,
}

#[derive(Clone, Debug)]
pub struct ExprBinType {
    pub id: NodeId,
    pub pos: Position,
    pub span: Span,

    pub op: BinOp,
    pub initializer: bool,
    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>,
}

#[derive(Clone, Debug)]
pub struct ExprLitCharType {
    pub id: NodeId,
    pub pos: Position,
    pub span: Span,

    pub value: char,
}

#[derive(Clone, Debug)]
pub struct ExprLitIntType {
    pub id: NodeId,
    pub pos: Position,
    pub span: Span,

    pub value: u64,
    pub base: IntBase,
    pub suffix: IntSuffix,
}

#[derive(Clone, Debug)]
pub struct ExprLitFloatType {
    pub id: NodeId,
    pub pos: Position,
    pub span: Span,

    pub value: f64,
    pub suffix: FloatSuffix,
}

#[derive(Clone, Debug)]
pub struct ExprLitStrType {
    pub id: NodeId,
    pub pos: Position,
    pub span: Span,

    pub value: String,
}

#[derive(Clone, Debug)]
pub struct ExprTemplateType {
    pub id: NodeId,
    pub pos: Position,
    pub span: Span,

    pub parts: Vec<Box<Expr>>,
}

#[derive(Clone, Debug)]
pub struct ExprLitBoolType {
    pub id: NodeId,
    pub pos: Position,
    pub span: Span,

    pub value: bool,
}

#[derive(Clone, Debug)]
pub struct ExprBlockType {
    pub id: NodeId,
    pub pos: Position,
    pub span: Span,

    pub stmts: Vec<Box<Stmt>>,
    pub expr: Option<Box<Expr>>,
}

#[derive(Clone, Debug)]
pub struct ExprSuperType {
    pub id: NodeId,
    pub pos: Position,
    pub span: Span,
}

#[derive(Clone, Debug)]
pub struct ExprSelfType {
    pub id: NodeId,
    pub pos: Position,
    pub span: Span,
}

#[derive(Clone, Debug)]
pub struct ExprIdentType {
    pub id: NodeId,
    pub pos: Position,
    pub span: Span,

    pub name: Name,
    pub type_params: Option<Vec<Type>>,
}

#[derive(Clone, Debug)]
pub struct ExprLambdaType {
    pub id: NodeId,
    pub pos: Position,
    pub span: Span,

    pub params: Vec<Param>,
    pub ret: Option<Box<Type>>,
    pub block: Box<Stmt>,
}

#[derive(Clone, Debug)]
pub struct ExprCallType {
    pub id: NodeId,
    pub pos: Position,
    pub span: Span,

    pub callee: Box<Expr>,
    pub args: Vec<Box<Expr>>,
}

impl ExprCallType {
    pub fn object(&self) -> Option<&Expr> {
        if let Some(type_param) = self.callee.to_type_param() {
            if let Some(dot) = type_param.callee.to_dot() {
                Some(&dot.lhs)
            } else {
                None
            }
        } else if let Some(dot) = self.callee.to_dot() {
            Some(&dot.lhs)
        } else {
            None
        }
    }

    pub fn object_or_callee(&self) -> &Expr {
        self.object().unwrap_or(&self.callee)
    }
}

#[derive(Clone, Debug)]
pub struct ExprParenType {
    pub id: NodeId,
    pub pos: Position,
    pub span: Span,

    pub expr: Box<Expr>,
}

#[derive(Clone, Debug)]
pub struct ExprMatchType {
    pub id: NodeId,
    pub pos: Position,
    pub span: Span,

    pub expr: Box<Expr>,
}

#[derive(Clone, Debug)]
pub struct ExprTypeParamType {
    pub id: NodeId,
    pub pos: Position,
    pub span: Span,

    pub callee: Box<Expr>,
    pub args: Vec<Type>,
}

#[derive(Clone, Debug)]
pub struct ExprPathType {
    pub id: NodeId,
    pub pos: Position,
    pub span: Span,

    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>,
}

#[derive(Clone, Debug)]
pub struct ExprDotType {
    pub id: NodeId,
    pub pos: Position,
    pub span: Span,

    pub lhs: Box<Expr>,
    pub rhs: Box<Expr>,
}
