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

pub(crate) use ao;
pub(crate) use ar;
