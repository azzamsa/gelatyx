#doitlive prompt: stev
#doitlive shell: /bin/zsh
#doitlive speed: 3

# doc.md: contains valid unformatted code-block
# doc1.md: contains invalid unformatted code-block
# doc2.md: contains valid formatted code-block
gelatyx --language lua doc.md doc1.md doc2.md --check
clear

gelatyx --language lua doc.md doc2.md --check
clear

# contains invalid doc
gelatyx --language lua doc.md doc1.md
clear

gelatyx --language lua doc.md doc2.md
clear
