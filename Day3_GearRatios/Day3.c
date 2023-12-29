#include "stdio.h"
#include "stdlib.h"
#include "string.h"

int part1(FILE* file){

	// Get the size of the grid assuming the grid is square
	int gridSize = 1;
	char ch;
	while(!feof(file)){
		ch = fgetc(file);
		if(ch == '\n'){
			gridSize++;
		}
	}
	rewind(file);

	char grid[gridSize][gridSize];

	printf("GridSize: %dx%d\n", gridSize, gridSize);

	// populate the grid
	int x = 0;
	int y = 0;
	for(int i=0; i<gridSize; i++){
		for(int j=0; j<gridSize; j++){
			if(fscanf(file, "%c", &grid[i][j]) != 1)
				exit(1);
			if(grid[i][j] == '\n'){
				if(fscanf(file, "%c", &grid[i][j]) != 1)
					exit(1);
			}
				
		}
	}

	// print the grid
	// for(int i=0; i<gridSize; i++){
	// 	for(int j=0; j<gridSize; j++){
	// 		printf("%c", grid[i][j]);
	// 	}
	// 	printf("\n");
	// }

	// print the coordinates of all the numbers
	for(int i=0; i<gridSize; i++){
		for(int j=0; j<gridSize; j++){
			if(grid[i][j] >= '0' && grid[i][j] <= '9'){
				printf("%c", grid[i][j]);
				// go right until you hit anything other than a
			}
		}
	}
	


    return 0;
}

int part2(FILE* file){
    
    return 0;
}

int main(){

    // Open the files
    FILE* file1;
    FILE* file2;
    file1 = fopen("input1.txt", "r");
    file2 = fopen("input1.txt", "r");
    if(file1 == NULL || file2 == NULL){
        printf("\nFile Error\n");
        exit(1);
    }

    int part1Answer = part1(file1);
    int part2Answer = part2(file2);

    printf("\nPart1 Answer: %d, Part2 Answer: %d\n", part1Answer, part2Answer);

    fclose(file1);
    fclose(file2);
    return 0;
}