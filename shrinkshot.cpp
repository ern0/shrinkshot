# ifdef WINDOWS
# define _WIN32_WINNT 0x0502
# include <windows.h>
# endif
# include <stdio.h>
# include <stdlib.h>
# include <unistd.h>
# include <string>
# include "upng/upng.h"
# define DEBUG (0)


class ShrinkShot {

	typedef char side_t;

	private:

		const char* sfnam;
		upng_t* upng;
		std::string imagemagick;
		std::string result;

		int width;
		int height;
		const unsigned char* buffer;
		int bytesPerPix;

		int gapCorrection;


	private: void about() {
		fprintf(
			stderr,
			"shrinkshot (2019.11.14 09:47) - shrink images by removing empty regions \n"
			"  (best use case: screenshot) \n"
			"  see https://github.com/ern0/shrinkshot \n"
		);
	} // about()


	private: void sig() {
		fprintf(stderr,"shrinkshot: ");
	} // sig()


	public:	int main(int argc,char* argv[]) {

		# ifdef WINDOWS
			AttachConsole(ATTACH_PARENT_PROCESS);
			freopen("CONOUT$","w",stdout);
			freopen("CONOUT$","w",stderr);
		# endif

		if (argc < 3) {
			about();
			fprintf(stderr,"usage:\n  shrinkshot screenshot.png result.png \n");
			exit(0);
		}

		prepare();
		load(argv[1]);
		proc();
		convert(argv[1],argv[2]);

		return 0;
	} // main()


	private: void load(char* fnam) {

		upng = upng_new_from_file(fnam);

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

			} // if okay

			else {
				upng_free(upng);

				sig();
				fprintf(stderr,"error processing file %s \n",fnam);
				exit(1);
			}

		} // if upng from file

		else {
			sig();
			fprintf(stderr,"error loading file %s \n",fnam);
			exit(1);
		}

	} // loadPng()


	private: void proc() {

		# if DEBUG >= 1
			fprintf(stderr,"%d x %d \n",width,height);
		# endif

		procSide('h',width,height);
		procSide('v',height,width);

		upng_free(upng);

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
		for (int outer = 1; outer < outerDim; outer++) {

			if (diffCount[outer] > 0) {

				if (gapPos == -1) continue; // no gap

				if (gapLen > 0) {

					bool reg = false;
					if (gapLen > 2) reg = true;

					if (reg) addResult(side,gapPos,gapLen);

					gapPos = -1;
					gapLen = -1;

				} // if gap close

			} // if diff

			else {

				if (gapPos == -1) {
					gapPos = outer;
					gapLen = 0;
				} // if gap open

				gapLen++; // gap inc

			} // else diff

		} // for outer

		free((void*)diffCount);
		free((void*)diffValue);

	} // procSide()


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


	private: void addResult(side_t side,int gapPos,int gapLen) {

		gapPos += 1;
		gapLen -= 1;

		gapPos -= gapCorrection;
		gapCorrection += gapLen;

		# if DEBUG >= 1
			fprintf(stderr,"gap side=%c pos=%d len=%d \n",side,gapPos,gapLen);
		# endif

		char buffer[200];

		if (side == 'h') {

			sprintf(
				buffer
				," -chop %dx0+%d+0"
				,gapLen
				,gapPos
			);

		} // if h

		else { // side == 'v'

			sprintf(
				buffer
				," -chop 0x%d+0+%d"
				,gapLen
				,gapPos
			);

		} // if v

		result.append(buffer);

	} // addResult()


	private: void prepare() {

		// TODO: check imagemagick installation (or die)

		# ifdef WINDOWS
			imagemagick = "magick convert";
		# else
			imagemagick = "convert";
		# endif

	} // prepare()


	private: void convert(const char* src, const char* dst) {

		std::string command;
		command.append(imagemagick);
		command.append(" ");
		command.append(result);
		command.append(" \"");
		command.append(src);
		command.append("\" \"");
		command.append(dst);
		command.append("\"");

		# if DEBUG > 0
			fprintf(stderr,"cmd: [%s]\n",command.c_str());
		# endif

		// TODO: change system() to something better
		system(command.c_str());

	} // convert()


}; // class ShrinkShot


int main(int argc,char* argv[]) {
	ShrinkShot shrinkshot;
	return shrinkshot.main(argc,argv);
} // main()
