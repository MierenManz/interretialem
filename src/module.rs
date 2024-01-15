type Function = ();
type Memory = ();
type Table = ();
type Global = ();
type Export = ();

pub(crate) struct Module<'a> {
    functions: Box<[Function]>,
    // tables: Box<[Table]>,
    memory: Option<Memory>,
    globals: Box<[Global]>,
    // datas: Box<[u8]>,
    // datas: Vec<(u32, u32)>,
    start: Option<&'a Function>,
    exports: Box<[Export]>,
}
