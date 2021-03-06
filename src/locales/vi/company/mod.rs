mod name;
mod prefix;

pub fn adjective() -> Option<&'static [&'static str]> {
    None
}

pub fn adjetive() -> Option<&'static [&'static str]> {
    None
}

pub fn bs_adjective() -> Option<&'static [&'static str]> {
    None
}

pub fn bs_noun() -> Option<&'static [&'static str]> {
    None
}

pub fn bs_verb() -> Option<&'static [&'static str]> {
    None
}

pub fn descriptor() -> Option<&'static [&'static str]> {
    None
}

pub fn legal_form() -> Option<&'static [&'static str]> {
    None
}

pub fn name() -> Option<&'static [&'static str]> {
    Some(self::name::NAME)
}

pub fn noun() -> Option<&'static [&'static str]> {
    None
}

pub fn prefix() -> Option<&'static [&'static str]> {
    Some(self::prefix::PREFIX)
}

pub fn suffix() -> Option<&'static [&'static str]> {
    None
}
