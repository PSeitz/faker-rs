pub static STREET_NAME: &'static [&'static str] = &[
    "#{street_root}#{street_suffix}",
    "#{street_prefix} #{street_root}#{street_suffix}",
    "#{Name.first_name}#{common_street_suffix}",
    "#{Name.last_name}#{common_street_suffix}",
];
