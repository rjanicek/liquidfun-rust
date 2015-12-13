#include <Box2D/Box2D.h>
#include "c_b2Particle.h"

extern "C" {

	b2ParticleColor* b2ParticleColor_New(uint8 r, uint8 g, uint8 b, uint8 a) {
		return new b2ParticleColor(r, g, b, a);
	}

	void b2ParticleColor_Delete(b2ParticleColor* p) {
		delete p;
	}

	bool b2ParticleColor_IsZero(b2ParticleColor* p) {
		return p->IsZero();
	}

	void b2ParticleColor_Set(b2ParticleColor* p, uint8 r, uint8 g, uint8 b, uint8 a) {
		p->Set(r, g, b, a);
	}

} // extern C

