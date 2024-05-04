import type { Metadata } from "next";
import { Inter } from "next/font/google";
import "./globals.css";
import CurrentUser from "./CurrentUser";

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
        <ol className="bm-4 flex my-2 mx-4">
          <li className="flex-auto w-64"><a href="/">Recommendations</a></li>
          <li className="flex-initial w-64"><CurrentUser /></li>
        </ol>

        {children}
      </body>
    </html>
  );
}
