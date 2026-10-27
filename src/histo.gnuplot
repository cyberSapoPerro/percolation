set terminal svg size 800,600
set output 'histo.svg'

set title 'Distribución de tamaños de clústeres'
set xlabel 'Tamaño del clúster'
set ylabel 'Frecuencia'

set border linewidth 1.2
set tics out
set grid ytics
set style fill solid 0.75
set boxwidth 0.9*10 absolute

binwidth = 10
binstart = 2

bin(x) = binwidth * floor((x - binstart) / binwidth) \
         + binstart + binwidth / 2.0

         set xrange [2:]

         plot 'histo.dat' using (bin($1)):(1.0) \
                                               smooth freq with boxes \
                                           lc rgb '#4584b6' title 'Clústeres'
