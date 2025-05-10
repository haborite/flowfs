use dioxus::prelude::*;
use crate::svg_icon;
use crate::model::{CellExtracted, Dir, FileProp};

/*
impl CellExtracted {
    fn to_file_tree(self: &Self) {
        for item in self.fileprops.iter() {

        }
    }
}
*/
/*
#[derive(PartialEq, Props, Clone)]
struct FileTreeProps {
    rootdir: Dir,
}
*/


#[component]
pub fn FileTree(rootdir: Dir) -> Element {
    if rootdir.name == "/" {
        rsx! {
            ul { class: "menu menu-xs bg-base-0 rounded-lg max-w-xs w-full",
                for fileprop in rootdir.fileprops {
                    li {
                        a { svg_icon::general_file {} {fileprop.name} }
                    }
                }
                for subdir in rootdir.dirs {
                    li {
                        FileTree { rootdir: subdir }
                    }
                }
            }
        }
    } else {
        rsx! {
            details { open: "false", 
                summary { svg_icon::directory {} {rootdir.name} }
                ul {
                    for fileprop in rootdir.fileprops {
                        li {
                            a { svg_icon::general_file {} {fileprop.name} }
                        }
                    }
                    for subdir in rootdir.dirs {
                        li {
                            FileTree { rootdir: subdir }
                        }
                    }
                }
            }
        }
    }
}

/*
#[component]
pub fn make_tree(fileprops: &Vec<FileSProp>) -> Element {
    rsx! {
        ul { class: "menu menu-xs bg-base-0 rounded-lg max-w-xs w-full",
            for fileprop in fileprops {
                if fileprop.path.len() == 1 {
                    li {
                        a { svg_icon::general_file{} format!("{}", fileprop.path[0]) }
                    }
                } else if fileprop.path.len() > 1 {
                    li {
                        details { open: "false", 
                            summary { svg_icon::directory {} fileprop.path[0] }
                            make_tree(fileprop.path[1..])
                        }
                    }
                }

            }
        }
    }
}
*/



#[component]
pub fn tree() -> Element {
    rsx! {
        ul { class: "menu menu-xs bg-base-0 rounded-lg max-w-xs w-full",
            li {
                a { svg_icon::document {} "resume.pdf" }
            }
            li {
                details { open: "false",
                    summary { svg_icon::directory {} "My Files" }
                    ul {
                        li {
                            a { svg_icon::general_file {} "Project-final.psd" }
                        }
                        li {
                            a { svg_icon::general_file {} "Project-final-2.psd" }
                        }
                        li {
                            details { open: "false",
                                summary { svg_icon::directory {} "Images" }
                                ul {
                                    li {
                                        a { svg_icon::image_file {} "Screenshot1.png" }
                                    }
                                    li {
                                        a { svg_icon::image_file {} "Screenshot2.png" }
                                    }
                                    li {
                                        details { open: "false",
                                            summary { svg_icon::directory {} "Others" }
                                            ul {
                                                li {
                                                    a { svg_icon::image_file {} "Screenshot3.png" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            li {
                a { svg_icon::document {} "reports-final-2.pdf" }
            }
        }
    }
}