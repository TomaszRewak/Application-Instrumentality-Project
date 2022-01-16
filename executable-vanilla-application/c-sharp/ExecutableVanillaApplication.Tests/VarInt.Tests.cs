using ExecutableVanillaApplication.Utils;
using NUnit.Framework;

namespace ExecutableVanillaApplication.Tests
{
	public class Tests
	{
		[Test]
		public void Test1()
		{
			VarInt varInt = new();

			Assert.IsFalse(varInt.IsComplete);
		}
	}
}