#include <Box2D/Box2D.h>
#include "c_b2Particle.h"

extern "C" {

	b2ParticleColor* b2ParticleColor_Zero() {
		return &b2ParticleColor_zero;
	}

} // extern C

