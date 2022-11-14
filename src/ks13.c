// OpenLibrary implementation
//
// bent this until I could get:
// move.l  $4,a6
// move.l  #name,a1
// moveq #0,d0
// jsr -552(a6)
// move.l  d0,base
//

void* _openlib(char* name, int version) {
    asm("movea.l (4), %a6");
    asm("movel %0, %%a1"::"r"(name));
    asm("movel %0, %%d0"::"r"(version));
    asm("jsr -552(%a6)"); // the pointer is in %d0 at that point so we car just "return"
}
void* _closelib(void* lib) {
    asm("movea.l (4), %a6");
    asm("movel %0, %%a1"::"r"(lib));
    asm("jsr -414(%a6)");
}

int _call(void* base, int lvo, int d1, int d2, int d3) {
    asm("movea.l %0, %%a6"::"r"(base+lvo));
    asm("movel %0, %%d1"::"r"(d1));
    asm("movel %0, %%d2"::"r"(d2));
    asm("movel %0, %%d3"::"r"(d3));
    asm("jsr (%a6)");
}