//  Simple_Texture2D.c
//    This is a simple example that draws a quad with a 2D
//    texture image. The purpose of this example is to demonstrate
//    the basics of 2D texturing
//  Based on https://github.com/danginsburg/opengles-book-samples/blob/master/LinuxX11/Chapter_9/Simple_Texture2D/Simple_Texture2D.c
#include <stdio.h>
#include <stdlib.h>
#include <assert.h>
#include <EGL/egl.h>
#include <GLES2/gl2.h>
#include "../lvgl.h"
#include "util.h"

typedef struct
{
    // Handle to a program object
    GLuint programObject;

    // Attribute locations
    GLint positionLoc;
    GLint texCoordLoc;

    // Sampler location
    GLint samplerLoc;

    // Texture handle
    GLuint textureId;

} UserData;

/* For 2x2 Image, 3 bytes per pixel (R, G, B):
{
    255, 0, 0,  // Red
    0, 255, 0,  // Green
    0, 0, 255,  // Blue
    255, 255, 0 // Yellow
}; */
#define BYTES_PER_PIXEL 3
GLubyte pixels[LV_HOR_RES_MAX * LV_VER_RES_MAX * BYTES_PER_PIXEL];

void put_px(uint16_t x, uint16_t y, uint8_t r, uint8_t g, uint8_t b, uint8_t a) {
    assert(x >= 0); assert(x < LV_HOR_RES_MAX);
    assert(y >= 0); assert(y < LV_VER_RES_MAX);
    int i = (y * LV_HOR_RES_MAX * BYTES_PER_PIXEL) + (x * BYTES_PER_PIXEL);
    pixels[i++] = r;  //  Red
    pixels[i++] = g;  //  Green
    pixels[i++] = b;  //  Blue
}

GLuint CreateTexture(void) {
    puts("Create texture...");
    GLuint texId;
    glGenTextures ( 1, &texId );
    glBindTexture ( GL_TEXTURE_2D, texId );

    glTexImage2D (
        GL_TEXTURE_2D, 
        0,  //  Level
        GL_RGB, 
        LV_HOR_RES_MAX,  //  Width
        LV_VER_RES_MAX,  //  Height 
        0,  //  Format 
        GL_RGB, 
        GL_UNSIGNED_BYTE, 
        pixels 
    );
    glTexParameteri ( GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR );
    glTexParameteri ( GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR );
    glTexParameteri ( GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_CLAMP_TO_EDGE );
    glTexParameteri ( GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_EDGE );
    return texId;
}

///
// Initialize the shader and program object
//
int Init(ESContext *esContext)
{
    puts("Init texture...");
    esContext->userData = malloc(sizeof(UserData));
    UserData *userData = esContext->userData;
    GLbyte vShaderStr[] =
        "attribute vec4 a_position;   \n"
        "attribute vec2 a_texCoord;   \n"
        "varying vec2 v_texCoord;     \n"
        "void main()                  \n"
        "{                            \n"
        "   gl_Position = a_position; \n"
        "   v_texCoord = a_texCoord;  \n"
        "}                            \n";

    GLbyte fShaderStr[] =
        "precision mediump float;                            \n"
        "varying vec2 v_texCoord;                            \n"
        "uniform sampler2D s_texture;                        \n"
        "void main()                                         \n"
        "{                                                   \n"
        "  gl_FragColor = texture2D( s_texture, v_texCoord );\n"
        "}                                                   \n";

    // Load the shaders and get a linked program object
    userData->programObject = esLoadProgram(vShaderStr, fShaderStr);

    // Get the attribute locations
    userData->positionLoc = glGetAttribLocation(userData->programObject, "a_position");
    userData->texCoordLoc = glGetAttribLocation(userData->programObject, "a_texCoord");

    // Get the sampler location
    userData->samplerLoc = glGetUniformLocation(userData->programObject, "s_texture");

    // Load the texture
    userData->textureId = CreateTexture();

    glClearColor(0.0f, 0.0f, 0.0f, 0.0f);
    return GL_TRUE;
}

///
// Draw a triangle using the shader pair created in Init()
//
void Draw(ESContext *esContext)
{
    assert(esContext != NULL);
    printf("Draw width=%d, height=%d...\n", esContext->width, esContext->height);

    UserData *userData = esContext->userData;
    assert(userData != NULL);

    float xOffset = 0.0;
    float yOffset = 0.0;
    float xScale = 1;
    float yScale = 1;
    GLfloat vVertices[] = {
        xOffset + -1.0f * xScale, yOffset + 1.0f * yScale, 0.0f,  // Position 0
        //-0.5f, 0.5f, 0.0f,  // Position 0
        0.0f, 0.0f,         // TexCoord 0

        xOffset + -1.0f * xScale, yOffset + -1.0f * yScale, 0.0f, // Position 1
        //-0.5f, -0.5f, 0.0f, // Position 1
        0.0f, 1.0f,         // TexCoord 1

        xOffset + 1.0f * xScale, yOffset + -1.0f * yScale, 0.0f,  // Position 2
        //0.5f, -0.5f, 0.0f,  // Position 2
        1.0f, 1.0f,         // TexCoord 2

        xOffset + 1.0f * xScale, yOffset + 1.0f * yScale, 0.0f,   // Position 3
        //0.5f, 0.5f, 0.0f,   // Position 3
        1.0f, 0.0f          // TexCoord 3
    };
    GLushort indices[] = {0, 1, 2, 0, 2, 3};

    // Set the viewport
    glViewport(0, 0, 
        esContext->width * LV_SCALE_RES, 
        esContext->height * LV_SCALE_RES);

    // Clear the color buffer
    glClear(GL_COLOR_BUFFER_BIT);

    // Use the program object
    glUseProgram(userData->programObject);

    // Load the vertex position
    glVertexAttribPointer(userData->positionLoc, 3, GL_FLOAT,
                          GL_FALSE, 5 * sizeof(GLfloat), vVertices);

    // Load the texture coordinate
    glVertexAttribPointer(userData->texCoordLoc, 2, GL_FLOAT,
                          GL_FALSE, 5 * sizeof(GLfloat), &vVertices[3]);

    glEnableVertexAttribArray(userData->positionLoc);
    glEnableVertexAttribArray(userData->texCoordLoc);

    // Bind the texture
    glActiveTexture(GL_TEXTURE0);
    glBindTexture(GL_TEXTURE_2D, userData->textureId);

    // Set the sampler texture unit to 0
    glUniform1i(userData->samplerLoc, 0);

    //  TODO
    glDrawElements(GL_TRIANGLES, 6, GL_UNSIGNED_SHORT, indices);
}

///
// Cleanup
//
void ShutDown(ESContext *esContext)
{
    UserData *userData = esContext->userData;

    // Delete texture object
    glDeleteTextures(1, &userData->textureId);

    // Delete program object
    glDeleteProgram(userData->programObject);

    free(esContext->userData);
}

#ifdef NOTUSED
int main(int argc, char *argv[])
{
    ESContext esContext;
    UserData userData;

    esInitContext(&esContext);
    esContext.userData = &userData;

    esCreateWindow(&esContext, "Simple Texture 2D", 320, 240, ES_WINDOW_RGB);

    if (!Init(&esContext))
        return 0;

    esRegisterDrawFunc(&esContext, Draw);

    esMainLoop(&esContext);

    ShutDown(&esContext);
}
#endif //  NOTUSED
