#include <stdio.h>
#include <stdlib.h>
#include <math.h>

void stampa(long MAX ,long m[MAX][3], int size){
    for(int i = 0; i < size; ++i){
        //printf("%lu^2+%lu^2=%lu^2", m[i][0], m[i][1], m[i][2]);
        printf("%lu^2+%lu^2=%lu^2\n", m[i][0], m[i][1], m[i][2]);

    }
}

void my_calc(long MAX){
    long x = 0;
    long y = 0;
    double z = 0;
    int cont = 0;

    long mat[MAX*2][3];

    for(long long i = 2; i < MAX; ++i){
        x = i*i;
        for(long long j = i; j < x; ++j){
            y = j*j;
            z = sqrt(x+y);

            if(z > MAX) break;
            if(z - (unsigned int) z != 0) continue;

            printf("%llu^2+%llu^2=%llu^2\n", i, j, (long long unsigned) z);


            // mat[cont][0] = i;
            // mat[cont][1] = j;
            // mat[cont][2] = z;

            ++cont;
        }
    }

    // stampa(MAX*2,mat, cont);

    printf("Trovati %d risultati\n", cont);
}

int main(int argc, char **argv){

    if(argc != 2){
        printf("usage: time %s LIMIT\n", argv[0]);
        exit(1);
    }

    long MAX = strtol(argv[1], NULL, 0);
    my_calc(MAX);

    return 0;
}