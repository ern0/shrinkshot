# include <stdio.h>
# include <stdlib.h>
# include <unistd.h>
# include "upng/upng.h"
# define RET

class ShrinkShot {

	private:
	
	int width;
	int height;
	const unsigned char* buffer;
	int bytesPerPix;

	int* diffCount;
	int* diffValue;

	int hShrinkPos;
	int hShrinkLength;
	int vShrinkPos;
	int vShrinkLength;


	private: void sig() {
		fprintf(stderr,"shrinkshot: ");
	} // sig()


	public:	int main(int argc,char* argv[]) {

		if (argc != 2) {
			fprintf(stderr,"specify filename \n");
			exit(0);
		}

		upng_t* upng;

		upng = upng_new_from_file(argv[1]);
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
				fprintf(stderr,"error processing file %s \n",argv[1]);
				exit(1);
			}

			upng_free(upng);

		} // if upng from file

		else {
			sig();
			fprintf(stderr,"error loading file %s \n",argv[1]);
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

		fprintf(stderr,"%d x %d \n",width,height);

		procSide(RET &hShrinkPos,RET &hShrinkLength,'h',width,height);
		procSide(RET &vShrinkPos,RET &vShrinkLength,'v',height,width);

	} // proc()


	private: void procSide(RET int* pos,RET int* length,char mod,int pri,int sec) {

	} // procSide()


}; // class ShrinkShot


int main(int argc,char* argv[]) {
	ShrinkShot shrinkshot;
	return shrinkshot.main(argc,argv);
} // main()
