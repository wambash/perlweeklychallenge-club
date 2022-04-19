#!/usr/bin/env raku

sub is-abecedarian($word) {
    $word eq $word.comb.sort.join
}

'../../../data/dictionary.txt'
andthen .IO.lines
andthen .grep: &is-abecedarian
#andthen .map: *.say
;

'../../../data/dictionary.txt'
andthen .IO.lines
andthen .categorize: *.comb
andthen .nodemap: *.unique.elems
andthen .Mix
andthen { $^abc.nodemap: $^abc.total / * }\
andthen $_.say
