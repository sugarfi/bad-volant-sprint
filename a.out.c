unsigned int main (unsigned int a,unsigned int b,) {
unsigned int c = a;
if (a > b) {
a = b;
b = c;
c = a;

}
while (c < b) {
c = c + 1;

}
return 0;

}
