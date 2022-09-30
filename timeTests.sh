
echo -en "
5_10000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_5_10000.zok.in tests/KR_5_10000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_5_10000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
5_110000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_5_110000.zok.in tests/KR_5_110000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_5_110000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
5_210000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_5_210000.zok.in tests/KR_5_210000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_5_210000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
5_310000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_5_310000.zok.in tests/KR_5_310000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_5_310000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
5_410000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_5_410000.zok.in tests/KR_5_410000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_5_410000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
5_510000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_5_510000.zok.in tests/KR_5_510000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_5_510000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
5_610000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_5_610000.zok.in tests/KR_5_610000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_5_610000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
5_710000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_5_710000.zok.in tests/KR_5_710000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_5_710000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
5_810000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_5_810000.zok.in tests/KR_5_810000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_5_810000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
5_910000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_5_910000.zok.in tests/KR_5_910000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_5_910000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
105_10000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_105_10000.zok.in tests/KR_105_10000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_105_10000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
105_110000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_105_110000.zok.in tests/KR_105_110000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_105_110000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
105_210000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_105_210000.zok.in tests/KR_105_210000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_105_210000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
105_310000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_105_310000.zok.in tests/KR_105_310000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_105_310000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
105_410000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_105_410000.zok.in tests/KR_105_410000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_105_410000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
105_510000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_105_510000.zok.in tests/KR_105_510000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_105_510000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
105_610000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_105_610000.zok.in tests/KR_105_610000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_105_610000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
105_710000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_105_710000.zok.in tests/KR_105_710000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_105_710000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
105_810000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_105_810000.zok.in tests/KR_105_810000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_105_810000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
105_910000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_105_910000.zok.in tests/KR_105_910000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_105_910000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
205_10000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_205_10000.zok.in tests/KR_205_10000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_205_10000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
205_110000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_205_110000.zok.in tests/KR_205_110000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_205_110000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
205_210000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_205_210000.zok.in tests/KR_205_210000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_205_210000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
205_310000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_205_310000.zok.in tests/KR_205_310000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_205_310000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
205_410000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_205_410000.zok.in tests/KR_205_410000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_205_410000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
205_510000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_205_510000.zok.in tests/KR_205_510000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_205_510000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
205_610000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_205_610000.zok.in tests/KR_205_610000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_205_610000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
205_710000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_205_710000.zok.in tests/KR_205_710000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_205_710000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
205_810000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_205_810000.zok.in tests/KR_205_810000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_205_810000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
205_910000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_205_910000.zok.in tests/KR_205_910000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_205_910000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
305_10000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_305_10000.zok.in tests/KR_305_10000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_305_10000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
305_110000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_305_110000.zok.in tests/KR_305_110000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_305_110000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
305_210000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_305_210000.zok.in tests/KR_305_210000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_305_210000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
305_310000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_305_310000.zok.in tests/KR_305_310000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_305_310000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
305_410000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_305_410000.zok.in tests/KR_305_410000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_305_410000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
305_510000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_305_510000.zok.in tests/KR_305_510000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_305_510000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
305_610000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_305_610000.zok.in tests/KR_305_610000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_305_610000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
305_710000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_305_710000.zok.in tests/KR_305_710000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_305_710000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
305_810000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_305_810000.zok.in tests/KR_305_810000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_305_810000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
305_910000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_305_910000.zok.in tests/KR_305_910000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_305_910000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
405_10000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_405_10000.zok.in tests/KR_405_10000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_405_10000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
405_110000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_405_110000.zok.in tests/KR_405_110000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_405_110000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
405_210000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_405_210000.zok.in tests/KR_405_210000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_405_210000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
405_310000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_405_310000.zok.in tests/KR_405_310000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_405_310000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
405_410000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_405_410000.zok.in tests/KR_405_410000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_405_410000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
405_510000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_405_510000.zok.in tests/KR_405_510000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_405_510000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
405_610000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_405_610000.zok.in tests/KR_405_610000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_405_610000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
405_710000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_405_710000.zok.in tests/KR_405_710000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_405_710000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
405_810000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_405_810000.zok.in tests/KR_405_810000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_405_810000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
405_910000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_405_910000.zok.in tests/KR_405_910000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_405_910000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
505_10000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_505_10000.zok.in tests/KR_505_10000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_505_10000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
505_110000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_505_110000.zok.in tests/KR_505_110000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_505_110000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
505_210000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_505_210000.zok.in tests/KR_505_210000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_505_210000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
505_310000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_505_310000.zok.in tests/KR_505_310000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_505_310000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
505_410000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_505_410000.zok.in tests/KR_505_410000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_505_410000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
505_510000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_505_510000.zok.in tests/KR_505_510000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_505_510000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
505_610000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_505_610000.zok.in tests/KR_505_610000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_505_610000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
505_710000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_505_710000.zok.in tests/KR_505_710000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_505_710000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
505_810000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_505_810000.zok.in tests/KR_505_810000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_505_810000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
505_910000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_505_910000.zok.in tests/KR_505_910000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_505_910000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
605_10000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_605_10000.zok.in tests/KR_605_10000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_605_10000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
605_110000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_605_110000.zok.in tests/KR_605_110000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_605_110000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
605_210000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_605_210000.zok.in tests/KR_605_210000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_605_210000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
605_310000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_605_310000.zok.in tests/KR_605_310000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_605_310000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
605_410000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_605_410000.zok.in tests/KR_605_410000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_605_410000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
605_510000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_605_510000.zok.in tests/KR_605_510000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_605_510000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
605_610000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_605_610000.zok.in tests/KR_605_610000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_605_610000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
605_710000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_605_710000.zok.in tests/KR_605_710000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_605_710000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
605_810000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_605_810000.zok.in tests/KR_605_810000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_605_810000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
605_910000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_605_910000.zok.in tests/KR_605_910000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_605_910000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
705_10000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_705_10000.zok.in tests/KR_705_10000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_705_10000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
705_110000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_705_110000.zok.in tests/KR_705_110000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_705_110000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
705_210000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_705_210000.zok.in tests/KR_705_210000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_705_210000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
705_310000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_705_310000.zok.in tests/KR_705_310000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_705_310000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
705_410000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_705_410000.zok.in tests/KR_705_410000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_705_410000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
705_510000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_705_510000.zok.in tests/KR_705_510000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_705_510000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
705_610000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_705_610000.zok.in tests/KR_705_610000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_705_610000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
705_710000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_705_710000.zok.in tests/KR_705_710000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_705_710000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
705_810000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_705_810000.zok.in tests/KR_705_810000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_705_810000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
705_910000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_705_910000.zok.in tests/KR_705_910000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_705_910000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
805_10000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_805_10000.zok.in tests/KR_805_10000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_805_10000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
805_110000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_805_110000.zok.in tests/KR_805_110000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_805_110000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
805_210000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_805_210000.zok.in tests/KR_805_210000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_805_210000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
805_310000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_805_310000.zok.in tests/KR_805_310000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_805_310000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
805_410000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_805_410000.zok.in tests/KR_805_410000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_805_410000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
805_510000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_805_510000.zok.in tests/KR_805_510000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_805_510000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
805_610000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_805_610000.zok.in tests/KR_805_610000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_805_610000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
805_710000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_805_710000.zok.in tests/KR_805_710000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_805_710000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
805_810000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_805_810000.zok.in tests/KR_805_810000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_805_810000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
805_910000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_805_910000.zok.in tests/KR_805_910000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_805_910000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
905_10000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_905_10000.zok.in tests/KR_905_10000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_905_10000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
905_110000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_905_110000.zok.in tests/KR_905_110000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_905_110000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
905_210000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_905_210000.zok.in tests/KR_905_210000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_905_210000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
905_310000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_905_310000.zok.in tests/KR_905_310000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_905_310000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
905_410000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_905_410000.zok.in tests/KR_905_410000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_905_410000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
905_510000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_905_510000.zok.in tests/KR_905_510000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_905_510000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
905_610000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_905_610000.zok.in tests/KR_905_610000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_905_610000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
905_710000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_905_710000.zok.in tests/KR_905_710000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_905_710000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
905_810000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_905_810000.zok.in tests/KR_905_810000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_905_810000.zok r1cs --action count >> timeList
echo -en "
" >> timeList

echo -en "
905_910000
" >> timeList
{ time ./target/release/examples/circ --inputs tests/KR_905_910000.zok.in tests/KR_905_910000.zok r1cs --action spartan > /dev/null ; } 2>> timeList
./target/release/examples/circ tests/KR_905_910000.zok r1cs --action count >> timeList
echo -en "
" >> timeList
