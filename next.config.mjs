/** @type {import('next').NextConfig} */
const nextConfig = {
  //output: 'export',
  webpack: (config, { webpack, isServer, nextRuntime}) => {
    if (!isServer && nextRuntime === 'nodejs') {
      config.plugins.push(
        new webpack.IgnorePlugin(/^aws-sdk$/),
      );
    }
    return config;
  },
  experimental: {
    //serverActions: true,
    serverComponentsExternalPackages: ['aws-sdk'],
  },
  eslint: {
    ignoreDuringBuilds: true,
  }
};

export default nextConfig;
