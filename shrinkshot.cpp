# include <stdio.h>
# include <stdlib.h>
# include <unistd.h>
# include "upng/upng.h"


int width;
int height;
const unsigned char* buffer;
int bytesPerPix;


int pix(int y,int x) {

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
} // pix()


void proc(upng_t* upng) {

	fprintf(stderr,"%d x %d :: %d \n",width,height,bytesPerPix);

	for (int n = 0; n < 33; n++) {
		printf("%d: %d \n",n,pix(n,n));
	}

} // proc()


void sig() {
	fprintf(stderr,"shrinkshot: ");
} // sig()


int main(int argc,char* argv[]) {

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