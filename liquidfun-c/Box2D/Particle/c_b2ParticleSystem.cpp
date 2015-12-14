#include <Box2D/Box2D.h>
#include "c_b2ParticleSystem.h"

extern "C" {

	int32 b2ParticleSystem_CreateParticle(b2ParticleSystem* self, const b2ParticleDef& def) {
		return self->CreateParticle(def);
	}

	void b2ParticleSystem_DestroyParticle(b2ParticleSystem* self, int32 index) {
		self->DestroyParticle(index);
	}

	int32 b2ParticleSystem_GetParticleCount(b2ParticleSystem* self) {
		return self->GetParticleCount();
	}

	b2ParticleSystem* b2ParticleSystem_GetNext(b2ParticleSystem* self) {
		return self->GetNext();
	}

	uint32 b2ParticleSystem_GetParticleFlags(b2ParticleSystem* self, const int32 index) {
		return self->GetParticleFlags(index);
	}

	b2Vec2* b2ParticleSystem_GetPositionBuffer(b2ParticleSystem* self) {
		return self->GetPositionBuffer();
	}

} // extern C

