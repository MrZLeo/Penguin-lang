%start E
%avoid_insert "ALPHABET"
%%
E: E 'PLUS' T
    | E 'MINUS' T
    | T ;

T: T 'MUL' F
    | T 'DIV' F
    | F ;

F: 'LB' E 'RB'
      | 'FUNC' 'LB' 'ALPHABET' 'RB'
      | 'ALPHABET';