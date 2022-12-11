use std::iter;
use std::str;

#[derive(PartialEq, Eq, Debug)]
enum Command<'a> {
    Cd(&'a str),
    Ls(Vec<FileSystem<'a>>),
}
#[derive(PartialEq, Eq, Debug)]
pub enum FileSystem<'a> {
    Directory(&'a str, Vec<FileSystem<'a>>),
    File(&'a str, i64),
}

#[derive(PartialEq, Eq, Debug)]
pub struct File<'a> {
    path: Vec<&'a str>,
    size: i64,
}

type SourcePointer<'a> = iter::Peekable<str::Lines<'a>>;

fn parse_ls<'a>(source: &mut SourcePointer<'a>) -> Vec<FileSystem<'a>> {
    let mut output: Vec<FileSystem> = Vec::new();
    while source.peek().map_or(false, |&line| !line.starts_with('$')) {
        let line: Vec<&str> = source.next().unwrap().split_whitespace().collect();
        output.push(if line[0] == "dir" {
            FileSystem::Directory(line[1], Vec::new())
        } else {
            FileSystem::File(line[1], str::parse(line[0]).unwrap())
        });
    }
    output
}

fn parse_command<'a>(source: &mut SourcePointer<'a>) -> Command<'a> {
    let command: Vec<&str> = source.next().unwrap().split_whitespace().collect();
    match command[1] {
        "cd" => Command::Cd(command[2]),
        "ls" => Command::Ls(parse_ls(source)),
        _ => panic!("Parse error"),
    }
}

fn parse_commands(s: &str) -> Vec<Command> {
    let mut source = s.lines().peekable();
    let mut commands = Vec::new();
    while source.peek() != None {
        commands.push(parse_command(&mut source));
    }
    commands
}

fn change_dir<'a>(dir: &'a str, wd: &mut Vec<&'a str>) {
    match dir {
        "/" => {}
        ".." => {
            wd.pop();
        }
        dir => {
            wd.push(dir);
        }
    };
}

fn push_files<'a>(
    wd: &[&'a str],
    fs: &Vec<FileSystem<'a>>,
    directories: &mut Vec<Vec<&'a str>>,
    files: &mut Vec<File<'a>>,
) {
    for file in fs {
        let mut tmp = wd.to_owned();
        match file {
            FileSystem::File(name, size) => {
                tmp.push(*name);
                files.push(File {
                    path: { tmp },
                    size: *size,
                });
            }
            FileSystem::Directory(name, _) => {
                tmp.push(*name);
                directories.push(tmp);
            }
        }
    }
}

fn generate_filesystem(commands: Vec<Command>) -> (Vec<Vec<&str>>, Vec<File>) {
    let mut wd: Vec<&str> = Vec::new();
    let mut directories: Vec<Vec<&str>> = vec![Vec::new()];
    let mut files: Vec<File> = Vec::new();
    for command in commands {
        match command {
            Command::Cd(name) => change_dir(name, &mut wd),
            Command::Ls(fs) => push_files(&wd, &fs, &mut directories, &mut files),
        }
    }
    (directories, files)
}

pub fn parse(s: &str) -> (Vec<Vec<&str>>, Vec<File>) {
    generate_filesystem(parse_commands(s))
}

fn directory_sizes((directories, files): &(Vec<Vec<&str>>, Vec<File>)) -> Vec<i64> {
    directories
        .iter()
        .map(|dir| {
            files
                .iter()
                .filter(|file| file.path.starts_with(dir))
                .map(|file| file.size)
                .sum::<i64>()
        })
        .collect()
}

pub fn part1(fs_info: &(Vec<Vec<&str>>, Vec<File>)) -> i64 {
    directory_sizes(fs_info)
        .iter()
        .filter(|&&x| x <= 100_000)
        .sum::<i64>()
}

pub fn part2(fs_info: &(Vec<Vec<&str>>, Vec<File>)) -> i64 {
    let sizes = directory_sizes(fs_info);
    let root_size = sizes.iter().max().unwrap();
    let unused_space = 70_000_000 - root_size;
    let space_needed = 30_000_000 - unused_space;
    *sizes.iter().filter(|&&x| x >= space_needed).min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "$ cd /\n\
            $ ls\n\
            dir a\n\
            14848514 b.txt\n\
            8504156 c.dat\n\
            dir d\n\
            $ cd a\n\
            $ ls\n\
            dir e\n\
            29116 f\n\
            2557 g\n\
            62596 h.lst\n\
            $ cd e\n\
            $ ls\n\
            584 i\n\
            $ cd ..\n\
            $ cd ..\n\
            $ cd d\n\
            $ ls\n\
            4060174 j\n\
            8033020 d.log\n\
            5626152 d.ext\n\
            7214296 k";
        assert_eq!(parse_commands(input), commands());
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&generate_filesystem(commands())), 95437);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&generate_filesystem(commands())), 24933642);
    }

    fn commands() -> Vec<Command<'static>> {
        vec![
            Command::Cd("/"),
            Command::Ls(vec![
                FileSystem::Directory("a", vec![]),
                FileSystem::File("b.txt", 14848514),
                FileSystem::File("c.dat", 8504156),
                FileSystem::Directory("d", vec![]),
            ]),
            Command::Cd("a"),
            Command::Ls(vec![
                FileSystem::Directory("e", vec![]),
                FileSystem::File("f", 29116),
                FileSystem::File("g", 2557),
                FileSystem::File("h.lst", 62596),
            ]),
            Command::Cd("e"),
            Command::Ls(vec![FileSystem::File("i", 584)]),
            Command::Cd(".."),
            Command::Cd(".."),
            Command::Cd("d"),
            Command::Ls(vec![
                FileSystem::File("j", 4060174),
                FileSystem::File("d.log", 8033020),
                FileSystem::File("d.ext", 5626152),
                FileSystem::File("k", 7214296),
            ]),
        ]
    }
}
