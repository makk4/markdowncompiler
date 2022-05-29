Grammar 0.1

Markdown basic syntax


Legend:

> {} = zero or more occurences  
> [] = optional


```
Body            :=  { Header | Paragraph | List | Code } .
Header          :=  "#" { "#" } " " Sentence <NEWLINE> | 
                    Sentence <NEWLINE> ( "=" { "=" } | "-" { "-" } ) .
Paragraph       :=  { ">" } Sentence ( <NEWLINE> | Linebreak ) .
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
UnorderedList   :=  ( "-" | "*" | "+" ) Text <NEWLINE> .
OrderedList     :=  Number { Number } "." Text <NEWLINE> .
HorizontalRule  :=  <NEWLINE> <NEWLINE> ( "***" { "*" } | "---" { "-" } |
                    "___" { "_" } ) .
Escaping        :=  "\" .
Letters         :=  ... .
Number          :=  "0" | "1" | "2" | ... | "9" .
```