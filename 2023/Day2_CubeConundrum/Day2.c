#include "stdio.h"
#include "stdlib.h"
#include "string.h"

int part1(FILE* fptr, int red, int green, int blue){

    int idSum = 0;
    int currGameId = 1;
    int gamePossible = 1;

    char* line = NULL;
    size_t size = 0;

    while(getline(&line, &size, fptr) != -1){
        // printf("getline: %s\n", line);

        // Skip the "Game x:"
        char* subset = line;
        while(*(subset++) != ':');
        
        // Parse until you reach the end of the line
        while(*subset != '\0'){
            subset++;
            // Split at the ; and \0
            while(*subset != ';' && *subset != '\0'){
                int qty;
                char color;
                if(sscanf(subset, "%d %c", &qty, &color) == 2){
                    switch(color){
                        case 'r':
                            if(qty > red) gamePossible = 0;
                            break;
                        case 'g':
                            if(qty > green) gamePossible = 0;
                            break;
                        case 'b':
                            if(qty > blue) gamePossible = 0;
                            break;
                        default:
                            break;
                    }
                    while ((*subset != '\0') && (*subset != ';') && (*subset != ','))
                        subset++;
                }else{
                    subset++;
                }
            }
        }

        if(gamePossible == 1){
            idSum += currGameId;
        }else{
            gamePossible = 1;
        }

        currGameId++;
    }

    free(line);
    return idSum;
}


int part2(FILE* fptr, int red, int green, int blue){

    int idSum = 0;
    int currGameId = 1;
    int gamePossible = 1;
    int power = 0;
    int powerSum = 0;

    char* line = NULL;
    size_t size = 0;

    while(getline(&line, &size, fptr) != -1){
        printf("getline: %s\n", line);

        int minRed = 0;
        int minGreen = 0;
        int minBlue = 0;

        // Skip the "Game x:"
        char* subset = line;
        while(*(subset++) != ':');
        
        // Parse until you reach the end of the line
        while(*subset != '\0'){
            subset++;
            // Split at the ; and \0
            while(*subset != ';' && *subset != '\0'){
                int qty;
                char color;
                if(sscanf(subset, "%d %c", &qty, &color) == 2){
                    switch(color){
                        case 'r':
                            if(qty > red) gamePossible = 0;
                            if(qty > minRed) minRed = qty;
                            break;
                        case 'g':
                            if(qty > green) gamePossible = 0;
                            if(qty > minGreen) minGreen = qty;
                            break;
                        case 'b':
                            if(qty > blue) gamePossible = 0;
                            if(qty > minBlue) minBlue = qty;
                            break;
                        default:
                            break;
                    }
                    while ((*subset != '\0') && (*subset != ';') && (*subset != ','))
                        subset++;
                }else{
                    subset++;
                }
            }
        }

        if(gamePossible == 1){
            idSum += currGameId;
        }else{
            gamePossible = 1;
        }

        currGameId++;

        power = minRed * minGreen * minBlue;
        powerSum += power;
        printf("power: %d, powerSum: %d\n", power, powerSum);
    }

    free(line);
    return powerSum;
}

int main(){

    // Open the input file
	FILE *fptr;
	fptr = (fopen("input.txt", "r"));
	if(fptr == NULL){
		printf("Error!");
		exit(1);
	}
    // Open the input file
	FILE *fptr2;
	fptr2 = (fopen("input.txt", "r"));
	if(fptr2 == NULL){
		printf("Error!");
		exit(1);
	}

    // Part1 Possible Cubes
    int red = 12;
    int green = 13;
    int blue = 14;
    int part1Answer = part1(fptr, red, green, blue);
    int part2Answer = part2(fptr2, red, green, blue);

    printf("\nPart1 Answer: %d, Part2 Answer: %d\n", part1Answer, part2Answer);

    fclose(fptr);
    fclose(fptr2);
    return 0;
}