﻿
namespace ChimoneyDotNet.Models.Payout;

public class InterledgerWalletPayout
{
    public string InterledgerWalletAddress { get; set; }
    public decimal ValueInUSD { get; set; }
}
