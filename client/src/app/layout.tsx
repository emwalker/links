import type { Metadata } from "next";
import { Inter } from "next/font/google";
import "./globals.css";

const inter = Inter({ subsets: ["latin"] });

export const metadata: Metadata = {
  title: "Recommendations",
  description: "Prototype app for exploring recommendations",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body className={inter.className}>
        <ol className="bm-4 float my-2 mx-4">
          <li><a href="/">Recommendations</a></li>
        </ol>

        {children}
      </body>
    </html>
  );
}
