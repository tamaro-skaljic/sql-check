use sql_check::check;

fn main() {
    let _sql = check!("SELECT * FROM this_table_does_not_exist_anywhere");
}
