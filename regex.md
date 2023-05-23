
abc
^abc
abc$
^abc$
^abc$/i
ab?c
ab+c
ab{2}c
ab{3,}c
ab{2,4}c
ab|cd
.
[abc]
[a-d]
[^abc]
[^a-d]
\. \* \[



regex = "SOL" + EXPR + "EOL" + INSENSITIVE?
expr = match *
match  = ("." | char | excaped-char | grup) mod?
mod = {} | * | + | ? 
{} = {num,num*}

Expression = 

match = char ili grupa ili 

match  +  |  * | ? | 
