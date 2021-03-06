#define _CRT_SECURE_NO_WARNINGS

#ifdef __clang__
#define STBIWDEF static inline
#endif
#define STB_IMAGE_WRITE_STATIC
#define STB_IMAGE_WRITE_IMPLEMENTATION
#include "stb_image_write.h"

#include <iostream>
#include <thread>
#include <mutex>

#include "rtweekend.h"

#include "camera.h"
#include "sphere.h"
#include "hittable_list.h"
#include "color.h"
#include "material.h"

hittable_list random_scene() {
    hittable_list world;

    auto ground_material = make_shared<lambertian>(color(0.5, 0.5, 0.5));
    world.add(make_shared<sphere>(point3(0, -1000, 0), 1000, ground_material));

    for (int a = -11; a < 11; a++) {
        for (int b = -11; b < 11; b++) {
            auto choose_mat = random_double();
            point3 center(a + 0.9 * random_double(), 0.2, b + 0.9 * random_double());

            if ((center - point3(4, 0.2, 0)).length() > 0.9) {
                shared_ptr<material> sphere_material;

                if (choose_mat < 0.8) {
                    // diffuse
                    auto albedo = color::random() * color::random();
                    sphere_material = make_shared<lambertian>(albedo);
                    world.add(make_shared<sphere>(center, 0.2, sphere_material));
                }
                else if (choose_mat < 0.95) {
                    // metal
                    auto albedo = color::random(0.5, 1);
                    auto fuzz = random_double(0, 0.5);
                    sphere_material = make_shared<metal>(albedo, fuzz);
                    world.add(make_shared<sphere>(center, 0.2, sphere_material));
                }
                else {
                    // glass
                    sphere_material = make_shared<dielectric>(1.5);
                    world.add(make_shared<sphere>(center, 0.2, sphere_material));
                }
            }
        }
    }

    auto material1 = make_shared<dielectric>(1.5);
    world.add(make_shared<sphere>(point3(0, 1, 0), 1.0, material1));

    auto material2 = make_shared<lambertian>(color(0.4, 0.2, 0.1));
    world.add(make_shared<sphere>(point3(-4, 1, 0), 1.0, material2));

    auto material3 = make_shared<metal>(color(0.7, 0.6, 0.5), 0.0);
    world.add(make_shared<sphere>(point3(4, 1, 0), 1.0, material3));

    return world;
}

color ray_color(const ray& r, const hittable& world, int depth) {
    hit_record rec;

    if (depth <= 0)
        return color(0, 0, 0);
    
    if (world.hit(r, 0.001, infinity, rec)) {
        ray scattered;
        color attenuation;

        if (rec.mat_ptr->scatter(r, rec, attenuation, scattered))
            return attenuation * ray_color(scattered, world, depth - 1);

        return color(0, 0, 0);
    }

    vec3 unit_direction = unit_vector(r.direction());
    auto t = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - t) * color(1.0, 1.0, 1.0) + t * color(0.5, 0.7, 1.0);
}

std::mutex _lock;
int current_iteration = 0;

void render(int j_start, int j_end, int height, int width, int samples, int max_depth,
            hittable_list world, camera cam, unsigned char* (&data) , int data_pos) {
    int current = data_pos;

    for (int j = j_start; j >= j_end; --j) {
        _lock.lock();
        ++current_iteration;
        std::cerr << "\rScanlines remaining: " << current_iteration << ' ' << std::flush;
        _lock.unlock();

        for (int i = 0; i < width; ++i) {
            color pixel_color(0, 0, 0);

            for (int s = 0; s < samples; ++s) {
                auto u = (i + random_double()) / (width - 1);
                auto v = (j + random_double()) / (height - 1);
                ray r = cam.get_ray(u, v);
                pixel_color += ray_color(r, world, max_depth);
            }

            write_color(data, current, pixel_color, samples);
            current += 3;
        }
    }
}

int main() {

    // Image

    const auto aspect_ratio = 3.0 / 2.0;
    const int image_width = 1200;
    const int image_height = static_cast<int>(image_width / aspect_ratio);
    const int samples_per_pixel = 250;
    const int max_depth = 25;

    int comp = 3;
    unsigned char* image_data = new unsigned char[image_width * image_height * comp];
    int iData = 0;

    // World
    auto world = random_scene();

    // Camera
    point3 lookfrom(13, 2, 3);
    point3 lookat(0, 0, 0);
    vec3 vup(0, 1, 0);
    auto dist_to_focus = 10.0;
    auto aperture = 0.1;

    camera cam(lookfrom, lookat, vup, 20, aspect_ratio, aperture, dist_to_focus);

    // Render
    int concurrency = std::thread::hardware_concurrency();

    if (concurrency == 0) concurrency = 2;

    std::vector<std::thread> threads;
    int step_row = image_height / concurrency;

    for (int i = 0; i < concurrency; ++i) {
        int current_row = image_height - step_row * i - 1;

        std::thread thread(render, current_row, current_row - step_row + 1, image_height,
            image_width, samples_per_pixel, max_depth, world, cam, std::ref(image_data),
            i * step_row * image_width * 3);
        
        threads.push_back(std::move(thread));
    }

    for (std::thread& thread : threads) {
        if (thread.joinable())
            thread.join();
    }

    //render(image_height - 1, 0, image_height, image_width, samples_per_pixel, max_depth,world, cam, std::ref(image_data), 0);
    //for (int j = image_height - 1; j >= 0; --j) {
    //    std::cerr << "\rScanlines remaining: " << j << ' ' << std::flush;

    //    for (int i = 0; i < image_width; ++i) {
    //        color pixel_color(0, 0, 0);

    //        for (int s = 0; s < samples_per_pixel; ++s) {
    //            auto u = (i + random_double()) / (image_width - 1);
    //            auto v = (j + random_double()) / (image_height - 1);
    //            ray r = cam.get_ray(u, v);
    //            pixel_color += ray_color(r, world, max_depth);
    //        }

    //        write_color(image_data, iData, pixel_color, samples_per_pixel);
    //        iData += 3;
    //    }
    //}

    stbi_write_png("image.png", image_width, image_height, comp, image_data, image_width * comp);

    delete[] image_data;
    std::cerr << "\nDone.\n";
}
