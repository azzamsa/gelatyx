#doitlive prompt: stev
#doitlive shell: /bin/zsh
#doitlive speed: 3

# doc.md: contains valid unformatted code blocks
# doc1.md: contains invalid unformatted code blocks
# doc2.md: contains valid formatted code blocks
gelatyx --language lua doc.md doc1.md doc2.md --check
clear

gelatyx --language lua doc.md doc2.md --check
clear

# contains invalid doc
gelatyx --language lua doc.md doc1.md
clear

gelatyx --language lua doc.md doc2.md
clear
