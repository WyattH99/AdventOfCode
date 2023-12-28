#include <stdio.h>
#include <stdlib.h>
#include <string.h>


void part1(FILE* fptr, int* calibrationSum){
	// Variables for each Line
	char* line = NULL;
	size_t len = 0;
	
	while(getline(&line, &len, fptr) != -1){

		// front pointer and back pointer
		char* frontptr = line;
		char* backptr = frontptr + strlen(line) - 2;
		int calibration = 0;

		// Find the First Number
		for(int i=0; i<strlen(line)-1; i++){
			
			if(*frontptr >= '0' && *frontptr <= '9'){
				break;
			}
			frontptr++;
		}
		// Find the Last Number
		for(int i=0; i<strlen(line)-1; i++){
			
			if(*backptr >= '0' && *backptr <= '9'){
				break;
			}
			backptr--;
		}

		calibration = ((*frontptr - '0') * 10) + (*backptr - '0');

		*calibrationSum += calibration;

	}


	free(line);
	return;

}

void part2(FILE* fptr, int* calibrationSum){
	// Variables for each Line
	char* line = NULL;
	size_t len = 0;
	char* one = "one";
	char* two = "two";
	char* three = "three";
	char* four = "four";
	char* five = "five";
	char* six = "six";
	char* seven = "seven";
	char* eight = "eight";
	char* nine = "nine";

	while(getline(&line, &len, fptr) != -1){

		// front pointer and back pointer
		char* frontptr = line;
		char* backptr = frontptr + strlen(line) - 2;
		int calibration = 0;
		int val1 = 0;
		int val2 = 0;

		

		// Find the First Number
		for(int i=0; i<strlen(line)-1; i++){
			
			if(*frontptr >= '0' && *frontptr <= '9'){
				val1 = *frontptr - '0';
				break;
			}

			// check for one, two, six
			if(i>=2){
				char substring[3];
				memcpy(substring, frontptr-2, 3);
				char* pointer = substring;
				if(strstr(pointer, one) != NULL){
					val1 = 1;
					break;
				}
				if(strstr(pointer, two) != NULL){
					val1 = 2;
					break;
				}
				if(strstr(pointer, six) != NULL){
					val1 = 6;
					break;
				}

			}

			// check for four, five, nine
			if(i>=3){
				char substring[4];
				memcpy(substring, frontptr-3, 4);
				char* pointer = substring;
				if(strstr(pointer, four) != NULL){
					val1 = 4;
					break;
				}
				if(strstr(pointer, five) != NULL){
					val1 = 5;
					break;
				}
				if(strstr(pointer, nine) != NULL){
					val1 = 9;
					break;
				}

			}
			// check for three, seven, eight
			if(i>=4){
				char substring[5];
				memcpy(substring, frontptr-4, 5);
				char* pointer = substring;
				if(strstr(pointer, three) != NULL){
					val1 = 3;
					break;
				}
				if(strstr(pointer, seven) != NULL){
					val1 = 7;
					break;
				}
				if(strstr(pointer, eight) != NULL){
					val1 = 8;
					break;
				}
			}

			frontptr++;
		}

		// Find the Last Number
		for(int i=0; i<strlen(line)-1; i++){
			
			if(*backptr >= '0' && *backptr <= '9'){
				val2 = *backptr - '0';
				break;
			}

			// check for one, two, six
			if(i>=2){
				char substring[3];
				memcpy(substring, backptr, 3);
				char* pointer = substring;
				if(strstr(pointer, one) != NULL){
					val2 = 1;
					break;
				}
				if(strstr(pointer, two) != NULL){
					val2 = 2;
					break;
				}
				if(strstr(pointer, six) != NULL){
					val2 = 6;
					break;
				}

			}

			// check for four, five, nine
			if(i>=3){
				char substring[4];
				memcpy(substring, backptr, 4);
				char* pointer = substring;
				if(strstr(pointer, four) != NULL){
					val2 = 4;
					break;
				}
				if(strstr(pointer, five) != NULL){
					val2 = 5;
					break;
				}
				if(strstr(pointer, nine) != NULL){
					val2 = 9;
					break;
				}

			}
			// check for three, seven, eight
			if(i>=4){
				char substring[5];
				memcpy(substring, backptr, 5);
				char* pointer = substring;
				if(strstr(pointer, three) != NULL){
					val2 = 3;
					break;
				}
				if(strstr(pointer, seven) != NULL){
					val2 = 7;
					break;
				}
				if(strstr(pointer, eight) != NULL){
					val2 = 8;
					break;
				}
			}

			backptr--;
		}

		calibration = (val1 * 10) + val2;

		*calibrationSum += calibration;
	}

	free(line);
	return;

}

int main(){
	
	// Open the input file
	FILE *fptr;
	fptr = (fopen("input.txt", "r"));
	if(fptr == NULL){
		printf("Error!");
		exit(1);
	}
	FILE *fptr2;
	fptr2 = (fopen("input.txt", "r"));
	if(fptr2 == NULL){
		printf("Error!");
		exit(1);
	}

	int part1Answer = 0;
	int part2Answer = 0; 

	part1(fptr, &part1Answer);
	part2(fptr2, &part2Answer);

	
	printf("Part 1 Answer: %d, Part 2 Answer: %d\n", part1Answer, part2Answer);



	fclose(fptr);
	fclose(fptr2);

	return 0;
}
