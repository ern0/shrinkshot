# include <stdio.h>
# include <stdlib.h>
# include <unistd.h>
# include "upng/upng.h"
# define DEBUG (0)


class ShrinkShot {

	typedef char side_t;

	private:
	
	const char* sfnam;

	int width;
	int height;
	const unsigned char* buffer;
	int bytesPerPix;

	int gapCorrection;


	private: void sig() {
		fprintf(stderr,"shrinkshot: ");
	} // sig()


	public:	int main(int argc,char* argv[]) {

		if (argc != 2) {
			fprintf(stderr,"specify filename \n");
			exit(0);
		}

		sfnam = argv[1];

		upng_t* upng;

		upng = upng_new_from_file(sfnam);
		if (upng != NULL) {
			upng_decode(upng);
			if (upng_get_error(upng) == UPNG_EOK) {

				width = upng_get_width(upng);
				height = upng_get_height(upng);
				buffer = upng_get_buffer(upng);
				int bpp = upng_get_bpp(upng);

				bool okay = false;
				if (bpp == 24) okay = true;
				if (bpp == 32) okay = true;
				if (!okay) {
					sig();
					fprintf(stderr,"unsupported bit-per-pixel: %d \n",bpp);
					exit(1);
				}

				bytesPerPix = bpp >> 3;
				proc(upng);
			
			} // if okay

			else {
				sig();
				fprintf(stderr,"error processing file %s \n",sfnam);
				exit(1);
			}

			upng_free(upng);

		} // if upng from file

		else {
			sig();
			fprintf(stderr,"error loading file %s \n",sfnam);
			exit(1);
		}

		return 0;
	} // main()


	private: int gray(int y,int x) {

		unsigned int offset = bytesPerPix * ( (y * width) + x );
		unsigned int value = buffer[offset];
		value += buffer[1 + offset];
		value += buffer[2 + offset];
		if (bytesPerPix == 4) {
			value += buffer[3 + offset];
		} else {
			value += buffer[1 + offset];
		}
		value = value >> 2;

		return value;
	} // gray()


	private: void proc(upng_t* upng) {

		# if DEBUG >= 1
			fprintf(stderr,"%d x %d \n",width,height);
		# endif

		procSide('h',width,height);
		procSide('v',height,width);

	} // proc()


	private: void procSide(side_t side,int outerDim,int innerDim) {

		int* diffCount = (int*)malloc(outerDim * sizeof(int));
		int* diffValue = (int*)malloc(outerDim * sizeof(int));

		for (int outer = 1; outer < outerDim; outer++) {
			
			diffCount[outer] = 0;
			diffValue[outer] = 0;

			for (int inner = 0; inner < innerDim; inner++) {

				int actual;
				int neighbour;

				if (side == 'h') {
					actual = gray(inner,outer);
					neighbour = gray(inner,outer - 1);
				} else {
					actual = gray(outer,inner);
					neighbour = gray(outer - 1,inner);					
				}

				int diff = actual - neighbour;
				if (diff < 0) diff = -diff;
				if (diff > 0) diffCount[outer]++;
				diffValue[outer] += diff;

			} // for inner

			# if DEBUG >= 2
				fprintf(stderr,"%c: outer=%d count=%d value=%d \n",side,outer,diffCount[outer],diffValue[outer]);
			# endif

		} // for outer

		int gapPos = -1;
		int gapLen = -1;
		gapCorrection = 0;

		int lastOuter = outerDim - 1;
		for (int outer = 1; outer < outerDim; outer++) {

			if ( (diffCount[outer] > 0) || (outer == lastOuter) ) {  // if diff: close gap if any

				if (gapPos != -1) {  // if there is an open gap
					if (gapLen > 0) {  // if gap should be closed

						bool reg = false;
						//if (gapLen > (outerDim / 6)) reg = true;
						if (gapLen > 2) reg = true; 

						if (reg) printResult(side,gapPos,gapLen);

						gapPos = -1;
						gapLen = -1;
					
					} // if gapLen
				} // if gapPos

			} // if diffCount[outer]

			else {  // else diff: start or continue gap
				
				if (gapPos == -1) {  // 
					gapPos = outer;
					gapLen = 0;
				} // if open a gap

				gapLen++; // gap inc

			} // else diff

		} // for outer

		free((void*)diffCount);
		free((void*)diffValue);

	} // procSide()


	private:
	void printResult(side_t side,int gapPos,int gapLen) {

		gapPos += 1;
		gapLen -= 1;
 
		gapPos -= gapCorrection;
		gapCorrection += gapLen;

		# if DEBUG >= 1
			fprintf(stderr,"gap side=%c pos=%d len=%d \n",side,gapPos,gapLen);
		# endif

		if (side == 'h') {

			printf(
				" -chop %dx0+%d+0 "
				,gapLen
				,gapPos
			);

		} // if h 

		else { // side == 'v'

			printf(
				" -chop 0x%d+0+%d "
				,gapLen
				,gapPos
			);

		} // if v

	} // printResult()


}; // class ShrinkShot


int main(int argc,char* argv[]) {
	ShrinkShot shrinkshot;
	return shrinkshot.main(argc,argv);
} // main()
