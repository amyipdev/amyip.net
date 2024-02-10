macro_rules! ar {
    ($from:expr, $fnc:ident, $cod:literal, $term:ident) => {{
        if let Ok(f) = $from {
            f
        } else {
            $fnc($term, $cod);
            return $cod;
        }
    }};
}

macro_rules! ao {
    ($from:expr, $fnc:ident, $cod:literal, $term:ident) => {{
        if let Some(f) = $from {
            f
        } else {
            $fnc($term, $cod);
            return $cod;
        }
    }};
}

macro_rules! axo {
    ($from:expr) => {{
        if let Some(f) = $from {
            f
        } else {
            return Err(());
        }
    }};
}

macro_rules! axr {
    ($from:expr) => {{
        if let Ok(f) = $from {
            f
        } else {
            return Err(());
        }
    }};
}

pub(crate) use ao;
pub(crate) use ar;
pub(crate) use axo;
pub(crate) use axr;
