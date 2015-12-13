#ifndef C_B2_PARTICLE
#define C_B2_PARTICLE

#ifdef __cplusplus
extern "C" {
#endif

	b2ParticleColor* b2ParticleColor_New(uint8 r, uint8 g, uint8 b, uint8 a);
	void b2ParticleColor_Delete(b2ParticleColor* p);
	bool b2ParticleColor_IsZero(b2ParticleColor* p);
	void b2ParticleColor_Set(b2ParticleColor* p, uint8 r, uint8 g, uint8 b, uint8 a);

#ifdef __cplusplus
} // extern C
#endif
#endif