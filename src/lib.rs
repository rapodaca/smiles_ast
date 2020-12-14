pub struct Atom {
    pub kind: AtomKind,
    pub links: Vec<Link>
}

pub enum AtomKind {
    Star,
    Aliphatic(Aliphatic),
    Aromatic(Aromatic),
    Bracket {
        isotope: Option<u16>,
        symbol: BracketSymbol,
        parity: Option<Parity>,
        hcount: Option<u8>,
        charge: Option<i8>,
        map: Option<u16>
    }
}

pub enum BracketSymbol {
    Star,
    Element(Element),
    Aromatic(BracketAromatic)
}

pub enum Link {
    Bond {
        kind: BondKind,
        target: Target
    },
    Split(Atom)
}

pub enum BondKind {
    Elided,
    Single,
    Double,
    Triple,
    Quadruple,
    Aromatic,
    Up,
    Down
}

pub enum Target {
    Join(u16),
    Atom(Atom)
}

pub enum Aliphatic {
    B, C, N, O, S, P, F, Cl, Br, I, At, Ts
}

pub enum Aromatic {
    B, C, N, O, S, P
}

pub enum BracketAromatic {
    B, C, N, O, S, P, Se, As
}

pub enum Element {
    //  0   1   2   3   4   5   6   7   8   9
            H,  He, Li, Be, B,  C,  N,  O,  F,  // 0
        Ne, Na, Mg, Al, Si, P,  S,  Cl, Ar, K,  // 1
        Ca, Sc, Ti, V,  Cr, Mn, Fe, Co, Ni, Cu, // 2
        Zn, Ga, Ge, As, Se, Br, Kr, Rb, Sr, Y,  // 3
        Zr, Nb, Mo, Tc, Ru, Rh, Pd, Ag, Cd, In, // 4
        Sn, Sb, Te, I,  Xe, Cs, Ba, La, Ce, Pr, // 5
        Nd, Pm, Sm, Eu, Gd, Tb, Dy, Ho, Er, Tm, // 6
        Yb, Lu, Hf, Ta, W,  Re, Os, Ir, Pt, Au, // 7
        Hg, Tl, Pb, Bi, Po, At, Rn, Fr, Ra, Ac, // 8
        Th, Pa, U,  Np, Pu, Am, Cm, Bk, Cf, Es, // 9
        Fm, Md, No, Lr, Rf, Db, Sg, Bh, Hs, Mt, // 10
        Ds, Rg, Cn, Nh, Fl, Mc, Lv, Ts, Og      // 11
}

pub enum Parity {
    Clockwise,
    Counterclockwise
}