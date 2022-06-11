# markdowncompiler

![example workflow](https://github.com/makk4/markdowncompiler/actions/workflows/rust.yml/badge.svg)

Markdown Transpiler written in Rust

Grammar 0.1

Markdown basic syntax


Legend:

> {} = zero or more occurences  
> [] = optional


```
Body            :=  { Header | Paragraph | List | Code } .
Header          :=  "#" { "#" } " " Sentence <NEWLINE> | 
                    Sentence <NEWLINE> ( "=" { "=" } | "-" { "-" } ) .
Paragraph       :=  { ">" } Sentence ( <NEWLINE> <NEWLINE> | Linebreak ) .
Sentence        :=  { Emphasis | Text } .
Emphasis        :=  Italic | Bold | "***" Text "***" | "___" Text "___" | 
                    "__*" Text "*__" | "**_" Text "_**" .
Italic          :=  "_" Text "_" | "*" Text "*" .
Bold            :=  "__" Text "__" | "**" Text "**" .
Text            :=  Image | Link | Letters .
Linebreak       :=  "  " <NEWLINE> .
Image           :=  "!" "[" Letters "]" "(" Letters ")"
Code            :=  ( "        " | <Tab> <Tab> ) Letters <NEWLINE> .
Link            :=  "[" Letters "]" "(" Letters ")" | "<" Letters ">" .
List            :=  UnorderedList | OrderedList
UnorderedList   :=  ( "-" | "*" | "+" ) Text <NEWLINE> <NEWLINE> .
OrderedList     :=  Number { Number } "." Text <NEWLINE> <NEWLINE> .
HorizontalRule  :=  <NEWLINE> <NEWLINE> ( "***" { "*" } | "---" { "-" } |
                    "___" { "_" } ) .
Escaping        :=  "\" .
Letters         :=  ... .
Number          :=  "0" | "1" | "2" | ... | "9" .
```
