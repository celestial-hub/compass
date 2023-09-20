pub(crate) type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;
