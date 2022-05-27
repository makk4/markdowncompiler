Grammar 0.1

Legend:

> {} := one or more occurences  
> [] := zero or more occurences


```
Body            :=  [ Header | Paragraph | List | Code ] .
Header          :=  { "#" } Sentence <NEWLINE> | 
                    Sentence <NEWLINE> ( { "=" } | { "-" } ) .
Paragraph       :=  [ ">" ] Sentence ( <NEWLINE> | Linebreak ) .
Sentence        :=  [ Emphasis | Text ] .
Emphasis        :=  Italic | Bold | "***" Text "***" | "___" Text "___" | 
                    "__*" Text "*__" | "**_" Text "_**" .
Italic          :=  "_" Text "_" | "*" Text "*" .
Bold            :=  "__" Text "__" | "**" Text "**" .
Text            :=  Image | Link | Letters .
Linebreak       :=  "  " <NEWLINE> .
Image           :=  "!" "[" Letters "]" "(" Letters ")"
Code            :=  ( "        " | <Tab> <Tab> ) Letters
Link            :=  "[" Letters "]" "(" Letters ")" | "<" Letters ">" .
List            :=  UnorderedList | OrderedList
UnorderedList   :=  [ ( "-" | "*" | "+" ) Text <NEWLINE> ] .
OrderedList     :=  { Number } "." Text <NEWLINE> .
HorizontalRule  :=  <NEWLINE> <NEWLINE> ( "***" [ "*" ] | "---" [ "-" ] |
                    "___" [ "_" ] )
Escaping        :=  "\" .
Letters         :=  ... .
Number          :=  "0" | "1" | "2" | ... | "9" .
```